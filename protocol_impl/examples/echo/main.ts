import * as log from "https://deno.land/std/log/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";
import {loadSync as loadEnvSync} from "https://deno.land/std/dotenv/mod.ts";

import type {HexString} from "https://deno.land/x/polkadot/util/types.ts";
import type {Keypair} from "https://deno.land/x/polkadot/util-crypto/types.ts";
import {u8aToString, hexToString, hexToU8a, stringToHex, u8aToHex} from "https://deno.land/x/polkadot/util/mod.ts";
import {cryptoWaitReady, ed25519PairFromSeed} from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import {encryptMessage, decryptMessage} from "./message_utils.ts"

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

const input = (Deno.args[0] ?? "").toString().trim();
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

try {
  const stringToEcho = parsedData.toString();
  if (stringToEcho.trim().length === 0) {
    renderAndExit(Result.Error, "TEXT_IS_BLANK");
  }

  const output = `Received: ${stringToEcho}`;
  renderAndExit(Result.Success, output)
} catch (e) {
  logger.error(e.message);
  renderPanic("UNCOVERED_EXCEPTION");
}

Deno.exit(0);
