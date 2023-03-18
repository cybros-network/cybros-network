// Generate test input for demo task executor

import { parse } from "https://deno.land/std/flags/mod.ts";
import { cryptoWaitReady } from "https://deno.land/x/polkadot/util-crypto/mod.ts";
import { u8aToHex, stringToHex } from "https://deno.land/x/polkadot/util/mod.ts";
import { Keyring } from "https://deno.land/x/polkadot/keyring/mod.ts";

const parsedArgs = parse(Deno.args, {
  alias: {
    mnemonic: "m",
    receiverPublicKey: "p"
  },
  boolean: [
    "e2e",
  ],
  string: [
    "mnemonic",
    "receiverPublicKey",
    "arg",
  ],
  default: {
    mnemonic: "//Alice",
    receiverPublicKey: "0x541b83ac2c498941f394f6e6aee2dba7fab935bb8be6e52bb0d55fe45f716c8e",
    arg: "Hello",
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

const args = [parsedArgs.arg]
if (parsedArgs.e2e) {
  console.log("E2E enabled");

  const encryptedArgs = u8aToHex(keyPair.encryptMessage(JSON.stringify(args), parsedArgs.receiverPublicKey));
  const rawInput = {
    e2e: true,
    senderPublicKey: u8aToHex(keyPair.publicKey),
    encryptedArgs,
  };
  const input = stringToHex(JSON.stringify(rawInput));

  console.log(`Raw input: ${JSON.stringify(rawInput, null, 2)}`);
  console.log(`Hexed input: ${input}`);
} else {
  const rawInput = {
    e2e: false,
    args,
  };
  const input = stringToHex(JSON.stringify(rawInput));

  console.log(`Raw input: ${JSON.stringify(rawInput, null, 2)}`);
  console.log(`Hexed input: ${input}`);
}
