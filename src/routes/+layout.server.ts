import { sha256 } from "@oslojs/crypto/sha2";
import type { LayoutServerLoad } from "./$types";
import prisma from "$lib/prisma";

const byte_to_hex: string[] = [];

for (let n = 0; n <= 0xff; ++n) {
  const hexOctet = n.toString(16).padStart(2, "0");
  byte_to_hex.push(hexOctet);
}

export const load: LayoutServerLoad = async (event) => {
  const source = event.getClientAddress() + event.request.headers.get("user-agent");
  const encoded = new TextEncoder().encode(source);
  const sha_bytes = sha256(encoded);

  // fastest way to go from Uint8Array -> hex string (as far as I'm aware)
  // would rather avoid doing the round trip and encode the sha as a string directly but idk how
  const octets = new Array(sha_bytes.length);
  for (let i = 0; i < sha_bytes.length; i++) {
    const byte = sha_bytes[i];
    octets.push(byte_to_hex[byte]);
  }
  const hash = octets.join('');

  // create the visitor (unless it already exists)
  // kinda hate that this is the only way to do it
  await prisma.visitor.upsert({ where: { hash }, create: { hash }, update: {} });
}