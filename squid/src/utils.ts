import * as ss58 from "@subsquid/ss58"

export const ss58Prefix = 42;

export function decodeSS58Address(bytes: Uint8Array) {
    return ss58.codec(ss58Prefix).encode(bytes)
}
