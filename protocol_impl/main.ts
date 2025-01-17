import {parse} from "https://deno.land/std@0.224.0/flags/mod.ts";
import * as log from "https://deno.land/std@0.224.0/log/mod.ts";
import * as path from "https://deno.land/std@0.224.0/path/mod.ts";
import {copySync} from "https://deno.land/std@0.224.0/fs/mod.ts";

import {BN, hexToU8a, isHex, u8aToHex, hexToString} from "npm:@polkadot/util";
import {cryptoWaitReady, mnemonicGenerate} from "npm:@polkadot/util-crypto";
import {KeyringPair} from "npm:@polkadot/keyring";
import {ApiPromise, HttpProvider, Keyring, WsProvider} from "npm:@polkadot/api";
import {Application, Router} from "https://deno.land/x/oak/mod.ts";

import {AnyJson} from "npm:@polkadot/types-codec";

const APP_NAME = "Cybros protocol reference implementation";
const APP_VERSION = "v0.0.1-dev";
const IMPL_BUILD_VERSION = 1;
const IMPL_BUILD_MAGIC_BYTES = new Uint8Array([0]);

const parsedArgs = parse(Deno.args, {
  alias: {
    "help": "h",
    "version": "v",
    "port": "p",
    "rpcUrl": "rpc-url",
    "workPath": "work-path",
    "jobExecutorPath": "job-executor-path",
    "ownerPhrase": "owner-phrase",
    "noHeartbeat": "no-heartbeat",
    "subscribePool": "subscribe-pool",
    "implId": "impl",
    "implSpecVersion": "spec",
  },
  boolean: [
    "help",
    "version",
    "noHeartbeat",
  ],
  string: [
    "rpcUrl",
    "bind",
    "port",
    "workPath",
    "jobExecutorPath",
    "ownerPhrase",
    "subscribePool",
  ],
  default: {
    rpcUrl: "ws://127.0.0.1:9944",
    bind: "127.0.0.1",
    port: "8080",
    workPath: path.dirname(path.fromFileUrl(import.meta.url)),
    jobExecutorPath: path.join(path.dirname(path.fromFileUrl(import.meta.url)), "examples/echo"),
    help: false,
    version: false,
    ownerPhrase: "",
    noHeartbeat: false,
    implId: 101,
    implSpecVersion: 1,
  },
});

async function prepareDirectory(path: string): Promise<boolean> {
  try {
    const pathStat = await Deno.stat(path);

    if (!pathStat.isDirectory) {
      return Promise.reject(`"${path} exists but not a directory."`);
    }
  } catch (e) {
    if (e instanceof Deno.errors.NotFound) {
      try {
        Deno.mkdirSync(path, { recursive: true });
      } catch (_e) {
        return Promise.reject(`Make directory "${path}" failed.`);
      }
    } else if (e instanceof Deno.errors.PermissionDenied) {
      return Promise.reject(`Requires read access to "${path}", run again with the --allow-read flag.`);
    }
  }

  return Promise.resolve(true);
}

function welcome() {
  console.log(`
${APP_NAME}

Warning: This is just a prototype implementation,
         in final product, it should be protected by TEE (Trusted Execution Environment) technology,
         which means the app's memories, instructions, and persists data will encrypt by CPU, and only the exact CPU can load them.
         Job deployers' can get an attestation for their job is running in a TEE.
         Without TEE protection, bad job may harm your OS, or you may discover sensitive data,
         so PLEASE DO NOT USE FOR PRODUCTION.
         `.trim());
}

function help() {
  console.log(`
Usage: deno run ./main.ts [OPTIONS]

Options:
    --rpc-url <WS_OR_HTTP_NODE_RPC_ENDPOINT>
      The RPC endpoint URL of Substrate node, default is "ws://127.0.0.1:9944"
    --work-path <PATH>
      The work path of the app, default is the app located path
    --subscribe-pool <POOL_ID>
      Subscribe jobs of the given pool
    --owner-phrase <PHRASE>
      Inject the owner wallet, will enable some shortcuts (e.g. auto do register if it hasn't).
      WARNING: Keep safe of your owner wallet
    --version
      Show version info.
    --help
`.trim());
}

function version() {
  console.log(`${APP_NAME} ${APP_VERSION} (${IMPL_BUILD_VERSION})`);
}

async function initializeLogger(logPath: string) {
  // logger not write to log instantly, need explict call `logger.handlers[0].flush()`
  await log.setup({
    handlers: {
      console: new log.ConsoleHandler("NOTSET"),
      file: new log.FileHandler("NOTSET", {
        filename: path.resolve(path.join(logPath, "computing_worker.log")),
        formatter: (rec) =>
          JSON.stringify(
            { ts: rec.datetime, topic: rec.loggerName, level: rec.levelName, msg: rec.msg },
          ),
      }),
    },
    loggers: {
      default: {
        level: "NOTSET",
        handlers: ["console"],
      },
      background: {
        level: "NOTSET",
        handlers: ["file", "console"],
      },
    },
  });
}

function loadOrCreateWorkerKeyPair(dataPath: string): KeyringPair | null {
  const secretFile = path.join(dataPath, "worker.secret");
  return (() => {
    try {
      const mnemonic = Deno.readTextFileSync(secretFile).trim();

      return new Keyring({type: "sr25519"}).addFromUri(mnemonic, {name: "Worker"});
    } catch (e) {
      if (e instanceof Deno.errors.NotFound) {
        const mnemonic = mnemonicGenerate(12);
        Deno.writeTextFileSync(secretFile, mnemonic);

        return new Keyring({type: "sr25519"}).addFromUri(mnemonic, {name: "Worker"});
      }

      return null;
    }
  })();
}

function createSubstrateApi(rpcUrl: string): ApiPromise | null {
  let provider = null;
  if (rpcUrl.startsWith("wss://") || rpcUrl.startsWith("ws://")) {
    provider = new WsProvider(rpcUrl);
  } else if (
    rpcUrl.startsWith("https://") || rpcUrl.startsWith("http://")
  ) {
    provider = new HttpProvider(rpcUrl);
  } else {
    return null;
  }

  return new ApiPromise({
    provider,
    throwOnConnect: true,
    throwOnUnknown: true,
    types: {
      OnlinePayload: {
        impl_id: "u32",
        impl_spec_version: "u32",
        impl_build_version: "u32",
        impl_build_magic_bytes: "BoundedVec<u8, 64>",
      },
      AttestationMethod: {
        _enum: ["OptOut"],
      },
      AttestationError: {
        _enum: ["Invalid", "Expired"],
      },
      Attestation: {
        _enum: {
          OptOut: null,
        },
      },
      JobResult: {
        _enum: [
          "Success",
          "Fail",
          "Error",
          "Panic",
        ],
      },
      JobOutput: "BoundedVec<u8, 2048>",
      Proof: "BoundedVec<u8, 2048>",
    },
  });
}

function createAttestation(api: ApiPromise, _payload: any) {
  return api.createType("Attestation", "OptOut")
}

enum WorkerStatus {
  Unregistered = "Unregistered",
  Registered = "Registered",
  Unresponsive = "Unresponsive",
  RequestingOffline = "RequestingOffline",
  Online = "Online",
  Offline = "Offline",
}

enum FlipFlopStage {
  Flip = "Flip",
  Flop = "Flop",
  // FlipToFlop = "FlipToFlop",
  // FlopToFlip = "FlopToFlip",
}

enum JobStatus {
  Pending = "Pending",
  Processing = "Processing",
  Processed = "Processed",
}

function numberToBalance(value: BN | string | number) {
  const bn1e12 = new BN(10).pow(new BN(12));
  return new BN(value.toString()).mul(bn1e12);
}

function balanceToNumber(value: BN | string) {
  const bn1e9 = new BN(10).pow(new BN(9));
  const bnValue = isHex(value) ? new BN(hexToU8a(value), "hex") : new BN(value.toString());
  // May overflow if the user too rich
  return bnValue.div(bn1e9).toNumber() / 1e3;
}

async function handleJob() {
  const logger = log.getLogger("background");
  const api = globalThis.substrateApi;
  const job = globalThis.locals.currentJob;

  if (job.status === JobStatus.Processing && globalThis.locals.sentProcessedJobAt) {
    logger.debug("Waiting processed job extrinsic finalize");

    return;
  }

  // TODO: Handle timeout or canceled

  if (globalThis.locals.runningJob) {
    console.log("Job is running...");
    return;
  }

  // console.log(job)
  console.log(`Processing ${job.id}`);

  const jobWorkPath = path.join(tempPath, "job"); // TODO:

  await prepareDirectory(jobWorkPath).catch((e) => {
    console.error(e.message);
    Deno.exit(1);
  });

  copySync(jobExecutorPath, jobWorkPath, { overwrite: true })

  // Run the job
  console.log(`Input: ${job.input}`);
  const command = new Deno.Command(Deno.execPath(), {
    args: [
      "run",
      "--no-prompt",
      "--allow-env",
      "--allow-net",
      "--allow-sys",
      `--allow-read=${jobWorkPath}`,
      `--allow-write=${jobWorkPath}`,
      path.join(jobWorkPath, "main.ts"),
      job.input,
    ],
    cwd: jobWorkPath,
    clearEnv: true,
    stdout: "piped",
    stderr: "piped",
  });
  const child = command.spawn();
  child.output().then(async ({code, stdout, stderr}) => {
    const out = new TextDecoder().decode(stdout).trim();
    console.log(out);
    const errorOut = new TextDecoder().decode(stderr).trim();
    console.error(errorOut);

    let parsedOut;
    try {
      parsedOut = JSON.parse(hexToString(out));
      console.log(parsedOut);
    } catch (e) {
      console.error(e.message);
    }

    const jobResult = api.createType("JobResult", parsedOut ? parsedOut.result : "Success");
    const jobOutput = out && out.length > 0 ? api.createType("JobOutput", out) : null;

    logger.info(`Sending "offchain_computing.submitJobResult()`);
    const txPromise = api.tx.offchainComputingPool.submitJobResult(globalThis.subscribePool, job.id, jobResult, jobOutput, null, null);
    logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
    const txHash = await txPromise.signAndSend(globalThis.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    globalThis.locals.sentProcessedJobAt = globalThis.latestBlockNumber;
    globalThis.locals.runningJob = undefined;
  });

  globalThis.locals.runningJob = child;
}

if (parsedArgs.version) {
  version();
  Deno.exit(0);
} else if (parsedArgs.help) {
  welcome();
  console.log("");
  help();
  Deno.exit(0);
} else {
  welcome();
  console.log("");
}

const implId = parsedArgs.implId ?? 1;
const implSpecVersion = parsedArgs.implSpecVersion ?? 1;

const jobExecutorPath = path.resolve(parsedArgs.jobExecutorPath);
const dataPath = path.resolve(path.join(parsedArgs.workPath, "data"));
const tempPath = path.resolve(path.join(parsedArgs.workPath, "tmp"));
const logPath = path.resolve(path.join(parsedArgs.workPath, "log"));
await prepareDirectory(dataPath).catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});
await prepareDirectory(tempPath).catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});
await prepareDirectory(logPath).catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});

console.log(`Impl id: ${implId}`);
console.log(`Impl spec version: ${implSpecVersion}`);
console.log(`Job executor path: ${jobExecutorPath}`);
console.log(`Data path: ${dataPath}`);
console.log(`Temp path: ${tempPath}`);
console.log(`Log path: ${logPath}`);

await initializeLogger(logPath).catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});

await cryptoWaitReady().catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});

const workerKeyPair = loadOrCreateWorkerKeyPair(dataPath);
if (workerKeyPair === null) {
  console.error("Can not load or create the worker wallet.");
  Deno.exit(1);
} else {
  console.log(`Worker address: ${workerKeyPair.address}`);
}

const ownerKeyPair = (() => {
  const ownerPhrase = parsedArgs.ownerPhrase.toString().trim();
  if (ownerPhrase === "") {
    return null;
  }

  try {
    return new Keyring({ type: "sr25519" }).addFromUri(ownerPhrase, { name: "The owner" });
  } catch (e) {
    console.error(`Owner phrase invalid: ${e.message}`);
    return null;
  }
})();
if (ownerKeyPair !== null) {
  console.log(`Owner: ${ownerKeyPair.address}`);
}

const api = createSubstrateApi(parsedArgs.rpcUrl);
if (api === null) {
  console.error(`Invalid RPC URL "${parsedArgs.rpcUrl}"`);
  Deno.exit(1);
}

api.on("error", (e) => {
  const logger = log.getLogger("background");
  logger.error(e.message);

  console.error(`Polkadot.js error: ${e.message}"`);
  Deno.exit(1);
});

await api.isReady.catch((e) => console.error(e));

const subscribePool = parseInt(parsedArgs.subscribePool);
if (isNaN(subscribePool)) {
  console.error("`--subscribe-pool` arg missing or invalid, worker won't listen any pool");
} else {
  console.log(`Listening pool ${subscribePool}`);
}

interface Locals {
  // Send by the owner, for convenient
  sentRegisterAt?: number;
  sentAuthorizePoolAt?: number;

  sentOnlineAt?: number;
  sentHeartbeatAt?: number;
  sentSubscribePoolAt?: number;
  sentTakeJobAt?: number;
  sentProcessedJobAt?: number;

  currentJob?: any;
  runningJob?: Deno.Process;
}

declare global {
  interface Window {
    workerKeyPair: KeyringPair;
    ownerKeyPair: KeyringPair | null;
    substrateApi: ApiPromise;

    noHeartbeat: boolean;
    subscribePool: number;

    finalizedBlockHash: string;
    finalizedBlockNumber: number;

    latestBlockHash: string;
    latestBlockNumber: number;

    workerStatus: WorkerStatus;
    attestedAt: number;

    locals: Locals;
  }
}

globalThis.workerKeyPair = workerKeyPair;
globalThis.ownerKeyPair = ownerKeyPair;
globalThis.substrateApi = api;

globalThis.noHeartbeat = parsedArgs.noHeartbeat;
globalThis.subscribePool = subscribePool;

globalThis.finalizedBlockNumber = 0;
globalThis.finalizedBlockHash = "";

globalThis.latestBlockNumber = 0;
globalThis.latestBlockHash = "";

globalThis.workerStatus = WorkerStatus.Unregistered;
globalThis.attestedAt = 0;

globalThis.locals = {};

// await globalThis.substrateApi.rpc.chain.subscribeFinalizedHeads(async (finalizedHeader) => {
await globalThis.substrateApi.rpc.chain.subscribeNewHeads(async (latestHeader) => {
  const logger = log.getLogger("background");
  const api = globalThis.substrateApi;

  // const finalizedBlockHash = finalizedHeader.hash.toHex();
  // const finalizedBlockNumber = finalizedHeader.number.toNumber();

  const finalizedHeader = await api.rpc.chain.getFinalizedHead();
  const finalizedBlockHash = finalizedHeader.hash.toHex();
  const finalizedBlockNumber = (await api.rpc.chain.getHeader(finalizedHeader)).number

  // const latestHeader = await api.rpc.chain.getHeader();
  const latestBlockHash = latestHeader.hash.toHex();
  const latestBlockNumber = latestHeader.number.toNumber();

  globalThis.finalizedBlockHash = finalizedBlockHash;
  globalThis.finalizedBlockNumber = finalizedBlockNumber;
  globalThis.latestBlockHash = latestBlockHash;
  globalThis.latestBlockNumber = latestBlockNumber;

  logger.debug(
    `best: #${latestBlockNumber} (${latestBlockHash}), finalized #${finalizedBlockNumber} (${finalizedBlockHash})`,
  );

  // const apiAt = await api.at(finalizedBlockHash);

  // Use the latest block instead of finalized one, so we don't delay handle any operation,
  // but confirm use finalized block
  const [workerInfo, flipOrFlop, inFlipSet, inFlopSet, { data: workerBalance }] = await Promise.all([
    api.query.offchainComputingInfra.workers(globalThis.workerKeyPair.address).then((v) =>
      v === null || v === undefined ? null : v.toJSON()
    ),
    api.query.offchainComputingInfra.flipOrFlop().then(stage => stage.toString()),
    api.query.offchainComputingInfra.flipSet(globalThis.workerKeyPair.address).then(v => v.isSome ? v.unwrap().toNumber() : null),
    api.query.offchainComputingInfra.flopSet(globalThis.workerKeyPair.address).then(v => v.isSome ? v.unwrap().toNumber() : null),
    api.query.system.account(globalThis.workerKeyPair.address),
  ]);

  if (workerInfo === null || workerInfo === undefined) {
    if (globalThis.locals.sentRegisterAt && globalThis.locals.sentRegisterAt >= finalizedBlockNumber) {
      logger.debug("Waiting register extrinsic finalize");

      return;
    }

    logger.warn("Worker hasn't registered");
    if (globalThis.ownerKeyPair !== null) {
      const initialDeposit = numberToBalance(1000);
      logger.info(`Sending "offchain_computing_workers.registerWorker(worker, implId, initialDeposit)`);
      const txPromise = api.tx.offchainComputingInfra.registerWorker(globalThis.workerKeyPair.address, implId, initialDeposit);
      logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
      const txHash = await txPromise.signAndSend(globalThis.ownerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      globalThis.locals.sentRegisterAt = latestBlockNumber;
    }

    return;
  } else if (globalThis.workerStatus === WorkerStatus.Unregistered && workerInfo.status === WorkerStatus.Registered) {
    logger.info("Worker has registered.");
    globalThis.locals.sentRegisterAt = undefined;
    globalThis.workerStatus = workerInfo.status;
    return;
  }

  if (
    workerInfo.status === WorkerStatus.Registered ||
    workerInfo.status === WorkerStatus.Unresponsive ||
    workerInfo.status === WorkerStatus.Offline
  ) {
    if (globalThis.locals.sentOnlineAt && globalThis.locals.sentOnlineAt >= finalizedBlockNumber) {
      logger.debug("Waiting online extrinsic finalize");

      return;
    }

    const payload = api.createType("OnlinePayload", {
      "impl_id": implId,
      "impl_spec_version": implSpecVersion,
      "impl_build_version": IMPL_BUILD_VERSION,
      "impl_build_magic_bytes": IMPL_BUILD_MAGIC_BYTES,
    });
    const payloadSig = globalThis.workerKeyPair.sign(payload.toU8a());
    const attestation = createAttestation(api, u8aToHex(payloadSig));

    logger.info(`Sending "offchain_computing_workers.online(payload, attestation)`);
    const txPromise = api.tx.offchainComputingInfra.online(payload, attestation);
    logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
    const txHash = await txPromise.signAndSend(globalThis.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    globalThis.locals.sentOnlineAt = latestBlockNumber;

    return;
  } else if (globalThis.workerStatus === WorkerStatus.Registered && workerInfo.status === WorkerStatus.Online) {
    logger.info("Worker is online.");
    globalThis.locals.sentOnlineAt = undefined;
    globalThis.workerStatus = workerInfo.status;
    return;
  }

  if (!globalThis.noHeartbeat) {
    const shouldHeartBeat = (
      flipOrFlop === FlipFlopStage.Flip && inFlipSet && latestBlockNumber >= inFlipSet
    ) || (
      flipOrFlop === FlipFlopStage.Flop && inFlopSet && latestBlockNumber >= inFlopSet
    );
    if (shouldHeartBeat && globalThis.locals.sentHeartbeatAt === undefined) {
      logger.info(`Sending "offchain_computing_workers.heartbeat()`);
      const txPromise = api.tx.offchainComputingInfra.heartbeat();
      logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
      const txHash = await txPromise.signAndSend(globalThis.workerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      globalThis.locals.sentHeartbeatAt = latestBlockNumber;
    } else if (finalizedBlockNumber > globalThis.locals.sentHeartbeatAt) {
      globalThis.locals.sentHeartbeatAt = undefined;
    }
  }

  globalThis.workerStatus = workerInfo.status;
  globalThis.attestedAt = workerInfo.attestedAt

  // Watch worker's balance
  const freeWorkerBalance = balanceToNumber(workerBalance.free);
  const workerBalanceThreshold = 10;
  if (freeWorkerBalance < workerBalanceThreshold) {
    logger.warn(`Worker's free balance nearly exhausted: ${freeWorkerBalance}`);

    if (globalThis.ownerKeyPair !== null) {
      const deposit = numberToBalance(workerBalanceThreshold);
      logger.info(`Sending "offchainComputingInfra.deposit('${globalThis.workerKeyPair.address}', '${deposit}')"`);
      const txPromise = api.tx.offchainComputingInfra.deposit(globalThis.workerKeyPair.address, deposit);
      logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
      const txHash = await txPromise.signAndSend(globalThis.ownerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
    }
  }

  // // We only handle finalized event
  // const events = await apiAt.query.system.events();
  // events.forEach(({ event }) => {
  //   if (event.section !== "offchainComputing") {
  //     return;
  //   }
  //   if (event.data.worker === undefined || event.data.worker.toString() !== globalThis.workerKeyPair.address) {
  //     return;
  //   }
  //
  //   console.log(event.toHuman());
  // });

  if (isNaN(globalThis.subscribePool)) {
    return;
  }

  const [invited, subscribed] = await Promise.all([
    api.query.offchainComputingPool.poolAuthorizedWorkers(globalThis.workerKeyPair.address, globalThis.subscribePool).then(v => v.isSome),
    api.query.offchainComputingPool.workerSubscribedPools(globalThis.workerKeyPair.address, globalThis.subscribePool).then(v => v.isSome),
  ]);

  if (!invited) {
    console.log(`Worker not added to ${globalThis.subscribePool} yet`);

    if (globalThis.locals.sentAuthorizePoolAt && globalThis.locals.sentAuthorizePoolAt >= finalizedBlockNumber) {
      logger.debug("Waiting authorize worker extrinsic finalize");

      return;
    }

    if (globalThis.ownerKeyPair !== null) {
      logger.info(`Sending "offchainComputingPool.authorizeWorker(poolId, worker)`);
      const txPromise = api.tx.offchainComputingPool.authorizeWorker(globalThis.subscribePool, globalThis.workerKeyPair.address);
      logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
      const txHash = await txPromise.signAndSend(globalThis.ownerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      globalThis.locals.sentAuthorizePoolAt = latestBlockNumber;
    }

    return;
  } else if (globalThis.locals.sentAuthorizePoolAt && invited) {
    console.log(`Worker has been invited to pool ${globalThis.subscribePool}.`)

    globalThis.locals.sentAuthorizePoolAt = undefined
  }

  if (!subscribed) {
    if (globalThis.locals.sentSubscribePoolAt && globalThis.locals.sentSubscribePoolAt >= finalizedBlockNumber) {
      logger.debug("Waiting subscribe pool extrinsic finalize");

      return;
    }

    logger.info(`Sending "offchainComputingPool.subscribePool(poolId)`);
    const txPromise = api.tx.offchainComputingPool.subscribePool(globalThis.subscribePool);
    logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
    const txHash = await txPromise.signAndSend(globalThis.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    globalThis.locals.sentSubscribePoolAt = latestBlockNumber;

    return;
  } else if (globalThis.locals.sentSubscribePoolAt && subscribed) {
    console.log(`Worker subscribed pool ${globalThis.subscribePool}.`)

    globalThis.locals.sentSubscribePoolAt = undefined
  }

  // if (globalThis.locals.sentTakeJobAt && globalThis.locals.sentTakeJobAt >= globalThis.finalizedBlockNumber) {
  //   logger.debug("Waiting take job extrinsic finalize");
  //
  //   return;
  // }

  if (globalThis.locals.sentProcessedJobAt) {
    // if (globalThis.locals.sentTakeJobAt >= globalThis.finalizedBlockNumber) {
    //   logger.debug("Waiting submit job result extrinsic finalize");
    // } else {
    //   globalThis.locals.sentTakeJobAt = undefined;
    //   globalThis.locals.sentProcessedJobAt = undefined;
    //   globalThis.locals.runningJob = undefined;
    //   globalThis.locals.currentJob = undefined;
    // }
    globalThis.locals.sentTakeJobAt = undefined;
    globalThis.locals.sentProcessedJobAt = undefined;
    globalThis.locals.runningJob = undefined;
    globalThis.locals.currentJob = undefined;
  }

  if (globalThis.locals.currentJob === undefined) {
    globalThis.locals.sentTakeJobAt = undefined;

    const jobs =
      (await api.query.offchainComputingPool.jobs.entries(globalThis.subscribePool))
        .map(([_k, job]) => job.toJSON());

    const jobsOfMine = jobs
      .filter((job: AnyJson) => job.status != JobStatus.Processed && job.assignee === globalThis.workerKeyPair.address)
      .sort((a: AnyJson, b: AnyJson) => a.id - b.id);
    if (jobsOfMine.length > 0) {
      console.log(`Jobs assign to me: ${jobsOfMine.map((i) => i.id)}`);
      const job = jobsOfMine[0];
      // console.log(job);

      if ((job && globalThis.locals.currentJob === undefined) || globalThis.locals.currentJob.id == job.id) {
        const input = (await api.query.offchainComputingPool.jobInputs(globalThis.subscribePool, job.id)).unwrapOr(null);
        // console.log(input);
        job.input = input !== null ? u8aToHex(input.data) : "";
        job.rawInput = input;
        // console.log(job.input);

        globalThis.locals.currentJob = job

        await handleJob();
      }
    }

    let assignableJobsCount = jobs.filter((job: AnyJson) => job.status === JobStatus.Pending && job.assignee === null && job.implSpecVersion == implSpecVersion ).length
    let myJobsCount = (await api.query.offchainComputingPool.counterForWorkerAssignedJobs(globalThis.workerKeyPair.address)).toNumber()
    if (assignableJobsCount > 0 && myJobsCount <= 1 && globalThis.locals.sentTakeJobAt === undefined) {
      logger.info("taking a new job");

      logger.info(`Sending "offchain_computing.take_job(${globalThis.subscribePool}, null, true, null)`);
      const txPromise = api.tx.offchainComputingPool.takeJob(globalThis.subscribePool, null, true, null);
      logger.debug(`Call hash: ${txPromise.inner.toHex()}`);
      const txHash = await txPromise.signAndSend(globalThis.workerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      globalThis.locals.sentTakeJobAt = globalThis.latestBlockNumber;

      return;
    } else if (globalThis.locals.sentTakeJobAt && globalThis.locals.sentTakeJobAt >= finalizedBlockNumber) {
      globalThis.locals.sentTakeJobAt = undefined;
    } else {
      // logger.info("No new job");
      return;
    }
  }
});

const router = new Router();
router.get("/", (ctx) => {
  ctx.response.body = {
    latestBlockNumber: globalThis.latestBlockNumber,
    latestBlockHash: globalThis.latestBlockHash,
    finalizedBlockNumber: globalThis.finalizedBlockNumber,
    finalizedBlockHash: globalThis.finalizedBlockHash,
    workerAddress: globalThis.workerKeyPair.address,
    workerPublicKey: u8aToHex(globalThis.workerKeyPair.publicKey),
    workerStatus: globalThis.workerStatus,
    attestedAt: globalThis.attestedAt,
    appVersion: APP_VERSION,
    implVersion: IMPL_BUILD_VERSION,
  };
});

const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

app.addEventListener(
  "listen",
  (_e) => console.log(`Listening on http://${parsedArgs.bind}:${parsedArgs.port}`),
);
await app.listen({ hostname: parsedArgs.bind, port: parseInt(parsedArgs.port), secure: false });
