import * as log from "https://deno.land/std/log/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";
import { loadSync as loadEnvSync } from "https://deno.land/std/dotenv/mod.ts";

import { u8aToString, hexToString, hexToU8a, stringToHex, u8aToHex } from "https://deno.land/x/polkadot/util/mod.ts";
import { cryptoWaitReady, ed25519PairFromSeed } from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import { encryptMessage, decryptMessage } from "./message_utils.ts"

const workPath = path.dirname(path.fromFileUrl(import.meta.url));

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

  console.log(stringToHex("Initialize error"));
  Deno.exit(1);
});

await initializeLogger(path.resolve(path.join(workPath, "run.log"))).catch((e) => {
  console.error(e.message);

  console.log(stringToHex("Initialize error"));
  Deno.exit(1);
});
const logger = log.getLogger("default");

const env = loadEnvSync();
const e2eKeyPair = function () {
  try {
    return ed25519PairFromSeed(hexToU8a(env.E2E_KEY_SEED));
  } catch (e) {
    logger.error(e.message);

    console.log(stringToHex("Initialize error"));
    Deno.exit(1);
  }
}()

const input = (Deno.args[0] ?? "").toString().trim();
const parsedInput = function (input) {
  if (input.length === 0) {
    console.log(stringToHex("Input is blank"));
    Deno.exit(1);
  }

  try {
    return JSON.parse(hexToString(input));
  } catch (e) {
    logger.error(e.message);

    console.log(stringToHex("Can't parse input"));
    Deno.exit(1);
  }
}(input);

const e2eRequired = parsedInput.e2e ?? false;
const parsedArgs = function (input, e2eRequired, keyPair) {
  try {
    const e2eRequired = input.e2e ?? false;
    if (!e2eRequired) {
      return input.args ?? [];
    }

    return JSON.parse(
      u8aToString(
        decryptMessage(hexToU8a(input.encryptedArgs), keyPair.secretKey, input.senderPublicKey)
      )
    );
  } catch (e) {
    logger.error(e.message);

    console.log(stringToHex("Can't decrypt input"));
    Deno.exit(1);
  }
}(parsedInput, e2eRequired, e2eKeyPair);

// Do echo

try {
  const stringToEcho = parsedArgs[0].toString();
  if (stringToEcho.trim().length === 0) {
    console.log(stringToHex("Input is blank"));
    Deno.exit(1);
  }

  const output = `Received: ${stringToEcho}`;
  if (e2eRequired) {
    const encryptedOutput = u8aToHex(encryptMessage(output, e2eKeyPair.secretKey, parsedInput.senderPublicKey));
    console.log(encryptedOutput);
  } else {
    console.log(stringToHex(output));
  }
} catch (e) {
  logger.error(e.message);

  console.log(stringToHex("Unexpect error"));
  Deno.exit(1);
}

Deno.exit(0);
