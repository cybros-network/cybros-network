import * as log from "https://deno.land/std/log/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";
import {loadSync as loadEnvSync} from "https://deno.land/std/dotenv/mod.ts";
import {decode as decodeBase64} from 'https://deno.land/std/encoding/base64.ts';
import {crypto, toHashString} from "https://deno.land/std/crypto/mod.ts"
import {parse as parsePrompt} from "./flags/mod.ts";

import type {HexString} from "https://deno.land/x/polkadot/util/types.ts";
import type {Keypair} from "https://deno.land/x/polkadot/util-crypto/types.ts";
import {u8aToString, hexToString, hexToU8a, stringToHex, u8aToHex} from "https://deno.land/x/polkadot/util/mod.ts";
import {cryptoWaitReady, ed25519PairFromSeed} from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import {encryptMessage, decryptMessage} from "./message_utils.ts"

import {Akord, Auth, NodeJs} from "npm:@akord/akord-js@4.3.0-beta.1";
import {AkordWallet} from "npm:@akord/crypto";

const workPath = path.dirname(path.fromFileUrl(import.meta.url));

enum Result {
  Success = "Success",
  Fail = "Fail",
  Error = "Error",
  Panic = "Panic",
}

function renderResult(result: Result, data?: unknown) {
  console.log(stringToHex(JSON.stringify({
    result: result,
    e2e: false,
    data: data ?? null,
  })));
}

function renderResultWithE2E(
    e2eKeyPair: Keypair,
    recipientPublicKey: HexString | string | Uint8Array,
    result: Result,
    data?: unknown
) {
  console.log(stringToHex(JSON.stringify({
    result,
    e2e: true,
    senderPublicKey: u8aToHex(e2eKeyPair.publicKey),
    encryptedData: data ? u8aToHex(encryptMessage(JSON.stringify(data), e2eKeyPair.secretKey, recipientPublicKey)) : null,
  })));
}

function renderPanic(code: string) {
  console.log(stringToHex(JSON.stringify({
    result: Result.Panic,
    code,
  })));
}

// Stdout will be the output that submit to chain, we could use log for debugging
async function initializeLogger(logFilename: string) {
  // logger not write to log instantly, need explict call `logger.handlers[0].flush()`
  await log.setup({
    handlers: {
      file: new log.handlers.FileHandler("NOTSET", {
        filename: logFilename,
        formatter: (rec) =>
            JSON.stringify(
                { ts: rec.datetime, topic: rec.loggerName, level: rec.levelName, msg: rec.msg },
            ),
      }),
    },
    loggers: {
      default: {
        level: "NOTSET",
        handlers: ["file"],
      },
    },
  });
}

await cryptoWaitReady().catch((e) => {
  console.error(e.message);

  renderPanic("INIT_CRYPTO_FAIL");
  Deno.exit(1);
});

await initializeLogger(path.resolve(path.join(workPath, "run.log"))).catch((e) => {
  console.error(e.message);

  renderPanic("INIT_LOGGER_FAIL");
  Deno.exit(1);
});
const logger = log.getLogger("default");

const env = loadEnvSync();
const e2eKeyPair = function () {
  try {
    return ed25519PairFromSeed(hexToU8a(env.E2E_KEY_SEED));
  } catch (e) {
    logger.error(e.message);

    renderPanic("LOAD_E2E_KEYPAIR_FAIL");
    Deno.exit(1);
  }
}()

const input = (Deno.args[0] ?? "").toString().replaceAll(/(\r\n|\n|\r)/gm, "").trim();
const parsedInput = function (input) {
  if (input.length === 0) {
    renderResult(Result.Error, "INPUT_IS_BLANK");
    Deno.exit(1);
  }

  try {
    return JSON.parse(hexToString(input));
  } catch (e) {
    logger.error(e.message);

    renderResult(Result.Error, "INPUT_CANT_PARSE");
    Deno.exit(1);
  }
}(input);
const parsedData = function (input, keyPair) {
  try {
    const e2eRequired = input.e2e ?? false;
    if (!e2eRequired) {
      return input.data ?? null;
    }

    return JSON.parse(
        u8aToString(
            decryptMessage(hexToU8a(input.encryptedData), keyPair.secretKey, input.senderPublicKey)
        )
    );
  } catch (e) {
    logger.error(e.message);

    renderResult(Result.Error, "ENCRYPTED_ARGS_CANT_DECRYPT");
    Deno.exit(1);
  }
}(parsedInput, e2eKeyPair);

const renderAndExit = function (result: Result, data: unknown) {
  if (parsedInput.e2e) {
    renderResultWithE2E(e2eKeyPair, parsedInput.senderPublicKey, result, data);
  } else {
    renderResult(result, data);
  }
  Deno.exit(result == Result.Success ? 0 : 1);
}

// Main stage

const parsedArgs = parsePrompt(
  parsedData.toString().trim().split(" "),
  {
    alias: {
      "steps": "step",
      "negativePrompt": "neg",
      "cfgScale": "cfg",
      "clipSkip": "clip",
      "enableHr": "hr",
      "hrUpscaler": "hr-upscaler",
      "hrScale": "hr-upscale",
      "restoreFaces": "restore-faces",
      "denoisingStrength": "denoising",
      "hrSecondPassSteps": "hr-steps",
      "etaNoiseSeedDelta": "ensd",
    },
    boolean: [
      "enableHr",
      "restoreFaces",
      "tiling",
    ],
    string: [
      "negativePrompt",
      "model",
      "sampler",
      "hrUpscaler",
    ],
    default: {
      "sampler": "k_lms",
      "cfgScale": 7,
      "seed": -1,
      "steps": 20,
      "width": 512,
      "height": 512,
      "clipSkip": 1,
      "etaNoiseSeedDelta": 0,
      "enableHr": false,
      "restoreFaces": false,
      "tiling": false,
      "hrUpscaler": "None",
      "denoisingStrength": 0.7,
      "hrScale": 2,
      "hrSecondPassSteps": 20,
    }
  }
);

const prompt = parsedArgs._1 ? parsedArgs._1.toString().trim() : "";
if (prompt.length === 0) {
  renderAndExit(Result.Error, "PROMPT_IS_BLANK");
}
const negativePrompt = parsedArgs.negativePrompt ? parsedArgs.negativePrompt.trim() : "";
const cfgScale = Number(parsedArgs.cfgScale);
if (cfgScale !== parsedArgs.cfgScale) {
  renderAndExit(Result.Error, "CFG_SCALE_INVALID");
} else if (cfgScale < 1) {
  renderAndExit(Result.Error, "CFG_SCALE_SMALLER_THAN_ONE");
  Deno.exit(1);
}
const seed = parseInt(parsedArgs.seed);
if (seed !== parsedArgs.seed) {
  renderAndExit(Result.Error, "SEED_NOT_INTEGER");
}
const steps = parseInt(parsedArgs.steps);
if (steps !== parsedArgs.steps) {
  renderAndExit(Result.Error, "STEPS_NOT_INTEGER");
} else if (steps < 1) {
  renderAndExit(Result.Error, "STEPS_TOO_SMALL");
} else if (steps > 150) {
  renderAndExit(Result.Error, "STEPS_TOO_LARGE");
}
const width = parseInt(parsedArgs.width);
if (width !== parsedArgs.width) {
  renderAndExit(Result.Error, "WIDTH_NOT_INTEGER");
} else if (width < 128) {
  renderAndExit(Result.Error, "WIDTH_TOO_SMALL");
} else if (width > 2048) {
  renderAndExit(Result.Error, "WIDTH_TOO_LARGE");
}
const height = parseInt(parsedArgs.height);
if (height !== parsedArgs.height) {
  renderAndExit(Result.Error, "HEIGHT_NOT_INTEGER");
} else if (height < 128) {
  renderAndExit(Result.Error, "HEIGHT_TOO_SMALL");
} else if (height > 2048) {
  renderAndExit(Result.Error, "HEIGHT_TOO_LARGE");
}
const clipSkip = parseInt(parsedArgs.clipSkip);
if (clipSkip !== parsedArgs.clipSkip) {
  renderAndExit(Result.Error, "CLIP_SKIP_NOT_INTEGER");
} else if (clipSkip < 1) {
  renderAndExit(Result.Error, "CLIP_SKIP_TOO_SMALL");
} else if (clipSkip > 12) {
  renderAndExit(Result.Error, "CLIP_SKIP_TOO_LARGE");
}
const etaNoiseSeedDelta = parseInt(parsedArgs.etaNoiseSeedDelta);
if (etaNoiseSeedDelta !== parsedArgs.etaNoiseSeedDelta) {
  renderAndExit(Result.Error, "ENSD_NOT_INTEGER");
}
const enableHr = parsedArgs.enableHr ?? false;
const restoreFaces = parsedArgs.restoreFaces ?? false;
const tiling = parsedArgs.tiling ?? false;

const denoisingStrength = Number(parsedArgs.denoisingStrength);
if (enableHr) {
  if (denoisingStrength !== parsedArgs.denoisingStrength) {
    renderAndExit(Result.Error, "DENOISING_STRENGTH_INVALID");
  } else if (denoisingStrength < 0) {
    renderAndExit(Result.Error, "DENOISING_STRENGTH_TOO_SMALL");
  } else if (denoisingStrength > 1) {
    renderAndExit(Result.Error, "DENOISING_STRENGTH_TOO_LARGE");
  }
}
const hrScale = Number(parsedArgs.hrScale);
if (enableHr) {
  if (hrScale !== parsedArgs.hrScale) {
    renderAndExit(Result.Error, "HR_SCALE_INVALID");
  } else if (hrScale < 1) {
    renderAndExit(Result.Error, "HR_SCALE_TOO_SMALL");
  } else if (hrScale > 4) {
    renderAndExit(Result.Error, "HR_SCALE_TOO_LARGE");
  }
}
const hrSecondPassSteps = parseInt(parsedArgs.hrSecondPassSteps);
if (hrSecondPassSteps) {
  if (hrSecondPassSteps !== parsedArgs.hrSecondPassSteps) {
    renderAndExit(Result.Error, "HR_SECOND_PASS_STEPS_NOT_INTEGER");
  } else if (hrSecondPassSteps < 1) {
    renderAndExit(Result.Error, "HR_SECOND_PASS_STEPS_TOO_SMALL");
  } else if (hrSecondPassSteps > 150) {
    renderAndExit(Result.Error, "HR_SECOND_PASS_STEPS_TOO_LARGE");
  }
}
const hrUpscaler = parsedArgs.hrUpscaler ?? "None";

const modelName = parsedArgs.model ?? "v2-1_768-ema-pruned";
const samplerName = parsedArgs.sampler;

let modelTitle: string | null = null;
let samplerTitle: string | null = null;

// Check model
try {
  const resp = await fetch(`${env.SD_API_BASE}/sd-models`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  const models = await resp.json();

  for (const item of models) {
    if (item.model_name == modelName) {
      modelTitle = item.title
      break;
    }
  }

  if (!modelTitle) {
    logger.error(`Model "${modelName}" not found`);
    renderAndExit(Result.Error, "MODEL_NOT_FOUND");
  }
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "SD_API_ERROR");
}

// Check sampler
try {
  const resp = await fetch(`${env.SD_API_BASE}/samplers`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  const samplers = await resp.json();

  for (const item of samplers) {
    if (item.name === samplerName || item.aliases.includes(samplerName)) {
      samplerTitle = item.name;
      break;
    }
  }

  if (!samplerTitle) {
    logger.error(`Sampler "${samplerTitle}" not found`);
    renderAndExit(Result.Error, "SAMPLER_NOT_FOUND");
  }
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "SD_API_ERROR");
}

// // Switch model and model level configurations
// try {
//   const _resp = await fetch(`${env.SD_API_BASE}/options`, {
//     method: "POST",
//     headers: {
//       "Content-Type": "application/json",
//     },
//     body: JSON.stringify({
//       "sd_model_checkpoint": modelTitle,
//       "CLIP_stop_at_last_layers": clipSkip,
//       "eta_noise_seed_delta": etaNoiseSeedDelta,
//     }),
//   });
// } catch (e) {
//   logger.error(e.meesage);
//   renderAndExit(Result.Error, "SD_API_ERROR");
// }

// Call txt2img
let image: Uint8Array;
let responsePayload: string;
let responsePayloadHash: string;
try {
  const requestPayload: {
    "override_settings": unknown,
    "override_settings_restore_afterwards": boolean,
    "prompt": string,
    "negative_prompt"?: string,
    "sampler_name": string,
    "cfg_scale": number,
    "seed": number,
    "steps": number,
    "width": number,
    "height": number
    "restore_faces": boolean,
    "tiling": boolean,
    "enable_hr": boolean,
    "hr_scale": number,
    "hr_upscaler": string,
    "hr_second_pass_steps": string,
    "denoising_strength": number,
  } = {
    override_settings: {
      "sd_model_checkpoint": modelTitle,
      "CLIP_stop_at_last_layers": clipSkip,
      "eta_noise_seed_delta": etaNoiseSeedDelta,
    },
    override_settings_restore_afterwards: true,
    prompt,
    negative_prompt: negativePrompt.length > 0 ? negativePrompt : undefined,
    sampler_name: samplerTitle,
    cfg_scale: cfgScale,
    seed,
    steps,
    width,
    height,
    restore_faces: restoreFaces,
    tiling,
    enable_hr: enableHr,
  };

  if (enableHr) {
    Object.assign({
      hr_scale: hrScale,
      hr_upscaler: hrUpscaler,
      hr_second_pass_steps: hrSecondPassSteps,
      denoising_strength: denoisingStrength,
    })
  }

  const resp = await fetch(`${env.SD_API_BASE}/txt2img`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestPayload),
  });
  responsePayload = await resp.text();
  responsePayloadHash = toHashString(await crypto.subtle.digest("BLAKE2S", new TextEncoder().encode(responsePayload)));
  const parsedResponsePayload = JSON.parse(responsePayload);
  image = decodeBase64(parsedResponsePayload.images[0]);
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "SD_API_ERROR");
}

Auth.configure({ storage: localStorage });
Auth.configure({ apiKey: env.AKORD_API_KEY });

const wallet = await AkordWallet.importFromBackupPhrase(env.AKORD_ACCOUNT_BACKUP_PHRASE);
const akord = await Akord.init(wallet);

// Upload image
let uploadedImageUrl: string;
try {
  const imageFileName = `${responsePayloadHash}.png`
  const imageFile = new NodeJs.File([image], imageFileName, "image/png", (new Date()).getTime())
  const {stackId} = await akord.stack.create(env.AKORD_VAULT_ID, imageFile, imageFileName);
  uploadedImageUrl = `https://arweave.net/${await akord.stack.getUri(stackId)}`;
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "ARWEAVE_UPLOAD_ERROR");
}

// Upload proof
let uploadedProofUrl: string;
try {
  const proofFileName = `${responsePayloadHash}.proof.json`
  const proofFile = new NodeJs.File([new TextEncoder().encode(responsePayload)], proofFileName, "application/json", (new Date()).getTime())
  const {stackId} = await akord.stack.create(env.AKORD_VAULT_ID, proofFile, proofFileName);
  uploadedProofUrl = `https://arweave.net/${await akord.stack.getUri(stackId)}`;
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "ARWEAVE_UPLOAD_ERROR");
}

// compatible with https://docs.opensea.io/docs/metadata-standards
const metadata = {
  name: "Yet another AI generated artwork",
  image: uploadedImageUrl,
  proof: uploadedProofUrl,
  proof_hash: `BLAKE2S|${responsePayloadHash}`,
}

// Upload metadata
let uploadedMetadataUrl: string;
try {
  const metadataFileName = `${responsePayloadHash}.metadata.json`
  const metadataFile = new NodeJs.File([new TextEncoder().encode(JSON.stringify(metadata))], metadataFileName, "application/json", (new Date()).getTime())
  const {stackId} = await akord.stack.create(env.AKORD_VAULT_ID, metadataFile, metadataFileName);
  uploadedMetadataUrl = `https://arweave.net/${await akord.stack.getUri(stackId)}`;
} catch (e) {
  logger.error(e.meesage);
  renderAndExit(Result.Error, "ARWEAVE_UPLOAD_ERROR");
}

renderAndExit(Result.Success, uploadedMetadataUrl);

// // Currently Deno lacking crypto support that Areweave needed, so just left the code here
// import Arweave from "npm:arweave";
// const arweave = Arweave.init({
//     host: 'arweave.net',// Hostname or IP address for a Arweave host
//     protocol: 'https',  // Network protocol http or https
//     timeout: 20000,     // Network request timeouts in milliseconds
//     logging: false,     // Enable network request logging
// });
//
// const key = {}; // JWT get from Arweave wallet
// const address = await arweave.wallets.jwkToAddress(key);
// const balance = arweave.ar.winstonToAr(await arweave.wallets.getBalance(address));
// console.log(`${address}: ${balance}`)
//
// const imageFile = Deno.readFileSync("./test.png");
// let tx = await arweave.createTransaction({
//     data: imageFile
// }, key);
// tx.addTag('Content-Type', 'image/png');
// await arweave.transactions.sign(tx, key);
// console.log(tx);
//
// const response = await arweave.transactions.post(tx);
// console.log(response);
