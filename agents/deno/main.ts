import { parse } from "https://deno.land/std/flags/mod.ts";
import * as log from "https://deno.land/std/log/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";
import { copySync } from "https://deno.land/std/fs/mod.ts";

import { BN, hexToU8a, isHex, u8aToHex, u8aToString, hexToString } from "https://deno.land/x/polkadot/util/mod.ts";
import { cryptoWaitReady, mnemonicGenerate } from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import { KeyringPair } from "https://deno.land/x/polkadot/keyring/types.ts";
import { ApiPromise, HttpProvider, Keyring, WsProvider } from "https://deno.land/x/polkadot/api/mod.ts";
import { Application, Router } from "https://deno.land/x/oak/mod.ts";

const APP_NAME = "Research computing worker";
const APP_VERSION = "v0.0.1-dev";
const IMPL_NAME = "deno".split("").map(c => c.charCodeAt(0));
const IMPL_VERSION = 1;

const parsedArgs = parse(Deno.args, {
  alias: {
    "help": "h",
    "version": "v",
    "port": "p",
    "rpcUrl": "rpc-url",
    "workPath": "work-path",
    "ownerPhrase": "owner-phrase",
    "refreshAttestationInterval": "refresh-attestation-interval",
    "noHeartbeat": "no-heartbeat",
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
    "ownerPhrase",
  ],
  default: {
    rpcUrl: "ws://127.0.0.1:9944",
    bind: "127.0.0.1",
    port: "8080",
    workPath: path.dirname(path.fromFileUrl(import.meta.url)),
    help: false,
    version: false,
    ownerPhrase: "",
    refreshAttestationInterval: 40000,
    noHeartbeat: false,
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
${APP_NAME} implementation in Deno.

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
Usage: deno run ./app.ts [OPTIONS]

Options:
    --rpc-url <WS_OR_HTTP_NODE_RPC_ENDPOINT>
      The RPC endpoint URL of Substrate node, default is "ws://127.0.0.1:9944"
    --work-path <PATH>
      The work path of the app, default is the app located path
    --owner-phrase <PHRASE>
      Inject the owner wallet, will enable some shortcuts (e.g. auto do register if it hasn't).
      WARNING: Keep safe of your owner wallet
    --version
      Show version info.
    --help
`.trim());
}

function version() {
  console.log(`${APP_NAME} ${APP_VERSION} (${IMPL_VERSION})`);
}

async function initializeLogger(logPath: string) {
  // logger not write to log instantly, need explict call `logger.handlers[0].flush()`
  await log.setup({
    handlers: {
      console: new log.handlers.ConsoleHandler("NOTSET"),
      file: new log.handlers.FileHandler("NOTSET", {
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
  const keyPair = (() => {
    try {
      const mnemonic = Deno.readTextFileSync(secretFile).trim();

      return new Keyring({ type: "sr25519" }).addFromUri(mnemonic, { name: "Worker" });
    } catch (e) {
      if (e instanceof Deno.errors.NotFound) {
        const mnemonic = mnemonicGenerate(12);
        Deno.writeTextFileSync(secretFile, mnemonic);

        return new Keyring({ type: "sr25519" }).addFromUri(mnemonic, { name: "Worker" });
      }

      return null;
    }
  })();

  return keyPair;
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
      Address: "AccountId",
      LookupSource: "AccountId",
      AttestationPayload: "BoundedVec<u8, 64000>",
      ExtraOnlinePayload: "BoundedVec<u8, 64000>",
      OnlinePayload: {
        impl_name: "[u8; 4]",
        impl_version: "u32",
        extra: "BoundedVec<u8, 64000>"
      },
      NonTEEAttestation: {
        issued_at: "u64",
        payload: "AttestationPayload"
      },
      AttestationMethod: {
        _enum: ["Root", "NonTEE"],
      },
      AttestationError: {
        _enum: ["Invalid", "Expired"],
      },
      Attestation: {
        _enum: {
          NonTEE: "NonTEEAttestation"
        },
      },
      WorkerStatus: {
        _enum: [
          "Registered",
          "RequestingOffline",
          "Online",
          "Offline",
        ],
      },
      WorkerInfo: {
        account: "AccountId",
        owner: "AccountId",
        reserved: "Balance",
        status: "WorkerStatus",
        impl_name: "[u8; 4]",
        impl_version: "u32",
        attestation_method: "Option<AttestationMethod>",
        attested_at: "BlockNumber",
      },
      JobResult: {
        _enum: [
          "Success",
          "Failed",
          "Errored",
        ],
      },
      JobOutput: "BoundedVec<u8, 128000>"
    },
  });
}

function createAttestation(api: ApiPromise, payload: any) {
  const attestation = api.createType("NonTEEAttestation", {
    issued_at: Date.now(),
    payload: payload,
  });
  return api.createType("Option<Attestation>", { "NonTEE": attestation })
}

enum WorkerStatus {
  Unregistered = "Unregistered",
  Registered = "Registered",
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
  Created = "Created",
  Started = "Started",
  Completed = "Completed",
  // Timeout = "Timeout",
  // Cancelled = "Cancelled",
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
  const api = window.substrateApi;
  const job = window.locals.currentJob;

  if (job.status === JobStatus.Started && window.locals.sentCompleteJobAt) {
    logger.debug("Waiting complete job extrinsic finalize");

    return;
  }

  // TODO: Handle timeout or canceled

  if (window.locals.runningJob) {
    console.log("Job is running...");
    return;
  }

  if (job.status === JobStatus.Created) {
    if (window.locals.sentStartJobAt && window.locals.sentStartJobAt >= window.finalizedBlockNumber) {
      logger.debug("Waiting start job extrinsic finalize");

      return;
    }

    logger.info("new job incoming");

    logger.info(`Sending "simple_computing.start_job()`);
    const txPromise = api.tx.simpleComputing.startJob();
    logger.debug(`Call hash: ${txPromise.toHex()}`);
    const txHash = await txPromise.signAndSend(window.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    window.locals.sentStartJobAt = window.latestBlockNumber;

    return;
  }

  if (job.status === JobStatus.Started && window.locals.sentStartJobAt !== undefined) {
    window.locals.sentStartJobAt = undefined;
  }

  // TODO: This is only a demo, the job should fetch from the chain
  // Basically I'm thinking
  // - People can upload job executables to the chain, maybe seal in a NFT
  // - The job executable should be a single file, normally it should be a compiled JS, maybe we should support tarball as well?
  // - Requires a validation for deployed job executable, the entry must be `main.js` or `main.ts`
  // - Some way to deploy to workers
  // - Job should has an unique JID
  // - Job should has a hard timeout, and (MAYBE) should keeping report when processing the job
  // - Job should has a metadata, for permission controls and declaration resources
  // - Copy the job executable to `tmp/${JID}/`
  // - Run the job in `tmp/${JID}/`, the job executable has full control of the directory (but it should have hard limit)
  // - Allow upload materials to somewhere because logs are valuable
  // - Clean up `tmp/${JID}/` when a job is finished and archived on chain

  console.log(job)
  console.log(hexToString(job.input))

  const jobExecName = "my_job.ts";
  const jobSource = path.join(dataPath, jobExecName);
  const jobWorkPath = path.join(tempPath, "my_job");

  await prepareDirectory(jobWorkPath).catch((e) => {
    console.error(e.message);
    Deno.exit(1);
  });

  copySync(jobSource, path.join(jobWorkPath, jobExecName), { overwrite: true })

  // Run the job
  const command = new Deno.Command(Deno.execPath(), {
    args: [
      "run",
      "--no-prompt",
      "--allow-env",
      "--allow-net",
      `--allow-read=${jobWorkPath}`,
      `--allow-write=${jobWorkPath}`,
      path.join(jobWorkPath, jobExecName),
      hexToString(job.input),
    ],
    cwd: jobWorkPath,
    clearEnv: true,
    stdout: "piped",
    stderr: "piped",
  });
  const child = command.spawn();
  child.output().then(async ({code, stdout, stderr}) => {
    const parsedOut = new TextDecoder().decode(stdout);
    const parsedErrorOut = new TextDecoder().decode(stderr);

    console.log(parsedOut);
    console.log(parsedErrorOut);

    const result = code === 0 ? "Success" : "Failed";
    const jobResult = api.createType("JobResult", result);
    const jobOutput = api.createType("Option<JobOutput>", parsedOut)

    logger.info(`Sending "simple_computing.complete_job()`);
    const txPromise = api.tx.simpleComputing.completeJob(jobResult, jobOutput);
    logger.debug(`Call hash: ${txPromise.toHex()}`);
    const txHash = await txPromise.signAndSend(window.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    window.locals.sentCompleteJobAt = window.latestBlockNumber;
  });

  window.locals.runningJob = child;
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

interface Locals {
  sentRegisterAt?: number;
  sentOnlineAt?: number;
  sentHeartbeatAt?: number;
  sentRefreshAttestationAt?: number;
  sentStartJobAt?: number;
  sentCompleteJobAt?: number;

  currentJob?: any;
  runningJob?: Deno.Process;
}

declare global {
  interface Window {
    workerKeyPair: KeyringPair;
    ownerKeyPair: KeyringPair | null;
    substrateApi: ApiPromise;

    refreshAttestationInterval: number;
    noHeartbeat: boolean;

    finalizedBlockHash: string;
    finalizedBlockNumber: number;

    latestBlockHash: string;
    latestBlockNumber: number;

    workerStatus: WorkerStatus;
    attestedAt: number;

    locals: Locals;
  }
}

window.workerKeyPair = workerKeyPair;
window.ownerKeyPair = ownerKeyPair;
window.substrateApi = api;

window.refreshAttestationInterval = parseInt(parsedArgs.refreshAttestationInterval);
if (isNaN(window.refreshAttestationInterval)) {
  window.refreshAttestationInterval = 40000;
}
window.noHeartbeat = parsedArgs.noHeartbeat;

window.finalizedBlockNumber = 0;
window.finalizedBlockHash = "";

window.latestBlockNumber = 0;
window.latestBlockHash = "";

window.workerStatus = WorkerStatus.Unregistered;
window.attestedAt = 0;

window.locals = {};

await window.substrateApi.rpc.chain.subscribeFinalizedHeads(async (finalizedHeader) => {
  const logger = log.getLogger("background");
  const api = window.substrateApi;

  const finalizedBlockHash = finalizedHeader.hash.toHex();
  const finalizedBlockNumber = finalizedHeader.number.toNumber();

  const latestHeader = await api.rpc.chain.getHeader();
  const latestBlockHash = latestHeader.hash.toHex();
  const latestBlockNumber = latestHeader.number.toNumber();

  window.finalizedBlockHash = finalizedBlockHash;
  window.finalizedBlockNumber = finalizedBlockNumber;
  window.latestBlockHash = latestBlockHash;
  window.latestBlockNumber = latestBlockNumber;

  logger.debug(
    `best: #${latestBlockNumber} (${latestBlockHash}), finalized #${finalizedBlockNumber} (${finalizedBlockHash})`,
  );

  const apiAt = await api.at(finalizedBlockHash);

  // Use the latest block instead of finalized one, so we don't delay handle any operation,
  // but confirm use finalized block
  const [workerInfo, flipOrFlop, inFlipSet, inFlopSet, { data: workerBalance }] = await Promise.all([
    api.query.computingWorkers.workers(window.workerKeyPair.address).then((v) =>
      v === null || v === undefined ? null : v.toJSON()
    ),
    api.query.computingWorkers.flipOrFlop().then(stage => stage.toString()),
    api.query.computingWorkers.flipSet(window.workerKeyPair.address).then(v => v.isSome ? v.unwrap().toNumber() : null),
    api.query.computingWorkers.flopSet(window.workerKeyPair.address).then(v => v.isSome ? v.unwrap().toNumber() : null),
    api.query.system.account(window.workerKeyPair.address),
  ]);

  if (workerInfo === null || workerInfo === undefined) {
    if (window.locals.sentRegisterAt && window.locals.sentRegisterAt >= finalizedBlockNumber) {
      logger.debug("Waiting register extrinsic finalize");

      return;
    }

    logger.warning("Worker hasn't registered");
    if (window.ownerKeyPair !== null) {
      const initialDeposit = numberToBalance(150);
      logger.info(`Sending "computing_workers.register(worker, initialDeposit)`);
      const txPromise = api.tx.computingWorkers.register(window.workerKeyPair.address, initialDeposit);
      logger.debug(`Call hash: ${txPromise.toHex()}`);
      const txHash = await txPromise.signAndSend(window.ownerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      window.locals.sentRegisterAt = latestBlockNumber;
    }

    return;
  } else if (window.workerStatus === WorkerStatus.Unregistered && workerInfo.status === WorkerStatus.Registered) {
    logger.info("Worker has registered.");
    window.locals.sentRegisterAt = undefined;
    window.workerStatus = workerInfo.status;
    return;
  }

  if (
    workerInfo.status === WorkerStatus.Registered ||
    workerInfo.status === WorkerStatus.Offline
  ) {
    if (window.locals.sentOnlineAt && window.locals.sentOnlineAt >= finalizedBlockNumber) {
      logger.debug("Waiting online extrinsic finalize");

      return;
    }

    const payload = api.createType("OnlinePayload", {
      "impl_name": IMPL_NAME,
      "impl_version": IMPL_VERSION,
      "payload": api.createType("AttestationPayload", [])
    });
    const payloadSig = window.workerKeyPair.sign(payload.toU8a());
    const attestation = createAttestation(api, u8aToHex(payloadSig));

    logger.info(`Sending "computing_workers.online(payload, attestation)`);
    const txPromise = api.tx.computingWorkers.online(payload, attestation);
    logger.debug(`Call hash: ${txPromise.toHex()}`);
    const txHash = await txPromise.signAndSend(window.workerKeyPair, { nonce: -1 });
    logger.info(`Transaction hash: ${txHash.toHex()}`);
    // TODO: Catch whether failed

    window.locals.sentOnlineAt = latestBlockNumber;

    return;
  } else if (window.workerStatus === WorkerStatus.Registered && workerInfo.status === WorkerStatus.Online) {
    logger.info("Worker is online.");
    window.locals.sentOnlineAt = undefined;
    window.workerStatus = workerInfo.status;
    return;
  }

  if (window.refreshAttestationInterval > 0) {
    if (window.locals.sentRefreshAttestationAt === undefined && latestBlockNumber > window.attestedAt + window.refreshAttestationInterval) {
      const payload = api.createType("OnlinePayload", {
        "impl_name": IMPL_NAME,
        "impl_version": IMPL_VERSION,
      });
      const payloadSig = window.workerKeyPair.sign(payload.toU8a());
      const attestation = createAttestation(api, u8aToHex(payloadSig));

      logger.info(`Sending "computing_workers.refreshAttestation(payload, attestation)`);
      const txPromise = api.tx.computingWorkers.refreshAttestation(payload, attestation);
      logger.debug(`Call hash: ${txPromise.toHex()}`);
      const txHash = await txPromise.signAndSend(window.workerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      window.locals.sentRefreshAttestationAt = latestBlockNumber;
    } else if (window.locals.sentRefreshAttestationAt && workerInfo.attestedAt >= window.locals.sentRefreshAttestationAt) {
      logger.info("Refreshed attestation.");
      window.locals.sentRefreshAttestationAt = undefined;
    }
  }

  if (!window.noHeartbeat) {
    const shouldHeartBeat = (
      flipOrFlop === FlipFlopStage.Flip && inFlipSet && latestBlockNumber >= inFlipSet
    ) || (
      flipOrFlop === FlipFlopStage.Flop && inFlopSet && latestBlockNumber >= inFlopSet
    );
    if (shouldHeartBeat && window.locals.sentHeartbeatAt === undefined) {
      logger.info(`Sending "computing_workers.heartbeat()`);
      const txPromise = api.tx.computingWorkers.heartbeat();
      logger.debug(`Call hash: ${txPromise.toHex()}`);
      const txHash = await txPromise.signAndSend(window.workerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
      // TODO: Catch whether failed

      window.locals.sentHeartbeatAt = latestBlockNumber;
    } else if (finalizedBlockNumber > window.locals.sentHeartbeatAt) {
      window.locals.sentHeartbeatAt = undefined;
    }
  }

  window.workerStatus = workerInfo.status;
  window.attestedAt = workerInfo.attestedAt

  // Watch worker's balance
  const freeWorkerBalance = balanceToNumber(workerBalance.free);
  const workerBalanceThreshold = 10;
  if (freeWorkerBalance < workerBalanceThreshold) {
    logger.warning(`Worker's free balance nearly exhausted: ${freeWorkerBalance}`);

    if (window.ownerKeyPair !== null) {
      const deposit = numberToBalance(workerBalanceThreshold);
      logger.info(`Sending "computingWorkers.deposit('${window.workerKeyPair.address}', '${deposit}')"`);
      const txPromise = api.tx.computingWorkers.deposit(window.workerKeyPair.address, deposit);
      logger.debug(`Call hash: ${txPromise.toHex()}`);
      const txHash = await txPromise.signAndSend(window.ownerKeyPair, { nonce: -1 });
      logger.info(`Transaction hash: ${txHash.toHex()}`);
    }
  }

  // // We only handle finalized event
  // const events = await apiAt.query.system.events();
  // events.forEach(({ event }) => {
  //   if (event.section !== "simpleComputing") {
  //     return;
  //   }
  //   if (event.data.worker === undefined || event.data.worker.toString() !== window.workerKeyPair.address) {
  //     return;
  //   }
  //
  //   console.log(event.toHuman());
  // });

  // We only handle finalized job
  const job = (await apiAt.query.simpleComputing.assignedJobs(window.workerKeyPair.address)).unwrapOr(null);
  if (job) {
    const jsonifyJob = job.toJSON();
    jsonifyJob.payload = u8aToString(job.payload);
    window.locals.currentJob = jsonifyJob

    await handleJob();
  } else if (window.locals.sentCompleteJobAt) {
    window.locals.sentCompleteJobAt = undefined;
    window.locals.runningJob = undefined;

    return;
  }
});

const router = new Router();
router.get("/", (ctx) => {
  ctx.response.body = {
    latestBlockNumber: window.latestBlockNumber,
    latestBlockHash: window.latestBlockHash,
    finalizedBlockNumber: window.finalizedBlockNumber,
    finalizedBlockHash: window.finalizedBlockHash,
    workerAddress: window.workerKeyPair.address,
    workerPublicKey: u8aToHex(window.workerKeyPair.publicKey),
    workerStatus: window.workerStatus,
    attestedAt: window.attestedAt,
    version: VERSION,
    implVersion: IMPL_VERSION,
  };
});

const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

app.addEventListener(
  "listen",
  (_e) => console.log(`Listening on http://${parsedArgs.bind}:${parsedArgs.port}`),
);
await app.listen({ hostname: parsedArgs.bind, port: parsedArgs.port, secure: false });
