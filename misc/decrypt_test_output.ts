// Generate test input for demo task executor

import { parse } from "https://deno.land/std/flags/mod.ts";
import { cryptoWaitReady } from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import { u8aToString } from "https://deno.land/x/polkadot/util/mod.ts";
import { Keyring } from "https://deno.land/x/polkadot/keyring/mod.ts";

const parsedArgs = parse(Deno.args, {
  alias: {
    mnemonic: "m",
    senderPublicKey: "p",
    output: "o",
  },
  string: [
    "mnemonic",
    "senderPublicKey",
    "output",
  ],
  default: {
    mnemonic: "//Alice",
    senderPublicKey: "0x541b83ac2c498941f394f6e6aee2dba7fab935bb8be6e52bb0d55fe45f716c8e",
    output: "0x129b1882848b580c7a202b1fb88a35e9d49699d592564aac1737cdc39238a750b42b733a6ec299e03a4a3c4bdb07f5ab55a1b8f572cde5"
  },
});

await cryptoWaitReady().catch((e) => {
  console.error(e.message);
  Deno.exit(1);
});

const keyPair = (() => {
  const operatorMnemonic = parsedArgs.mnemonic.toString().trim();
  if (operatorMnemonic === undefined || operatorMnemonic === "") {
    return null;
  }

  try {
    return new Keyring({ type: "ed25519" }).addFromUri(operatorMnemonic);
  } catch (e) {
    console.error(`Mnemonic invalid: ${e.message}`);
    return null;
  }
})();
if (keyPair !== null) {
  console.log(`Operator: ${keyPair.address}`);
} else {
  console.error("Mnemonic is required.")
  Deno.exit(1);
}

const decryptedOutput = u8aToString(keyPair.decryptMessage(parsedArgs.output, parsedArgs.senderPublicKey));
console.log(`Decrypted output: ${decryptedOutput}`)
