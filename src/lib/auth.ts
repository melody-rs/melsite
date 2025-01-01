import type { User, Session } from "@prisma/client"
import { encodeBase32, encodeHexLowerCase } from "@oslojs/encoding"
import { sha256 } from "@oslojs/crypto/sha2";
import prisma from "$lib/prisma";

export function generate_session_token(): string {
  const bytes = new Uint8Array(20);
  crypto.getRandomValues(bytes);
  const token = encodeBase32(bytes);
  return token;
}

// 1 hour (in milliseconds)
const expiration_factor = 1000 * 60 * 60;
const half_expiration_factor = expiration_factor / 2;

export async function create_session(token: string, user_id: number): Promise<Session> {
  const encoded = new TextEncoder().encode(token);
  const session_id = encodeHexLowerCase(sha256(encoded));
  const session: Session = {
    id: session_id,
    user_id,
    expires_at: new Date(Date.now() + expiration_factor)
  };
  await prisma.session.create({ data: session });
  return session;
}

export type ValidateResult = | { session: Session; user: User } | { session: null; user: null }
export async function validate_session_token(token: string): Promise<ValidateResult> {
  const encoded = new TextEncoder().encode(token);
  const session_id = encodeHexLowerCase(sha256(encoded));
  const result = await prisma.session.findUnique({
    where: { id: session_id },
    include: { user: true }
  });
  // no session found
  if (result === null) return { session: null, user: null };

  const { user, ...session } = result;
  // if its expired, remove it
  if (Date.now() >= session.expires_at.getTime()) {
    await invalidate_session(session_id);
    return { session: null, user: null };
  }

  if (Date.now() >= session.expires_at.getTime() - half_expiration_factor) {
    // extend session time
    session.expires_at = new Date(Date.now() + expiration_factor);
    await prisma.session.update({
      where: { id: session.id },
      data: { expires_at: session.expires_at }
    });
  }
  return { session, user };
}

export async function invalidate_session(session_id: string): Promise<void> {
  await prisma.session.delete({
    where: { id: session_id }
  });
}

export function user_is_admin(user: User | null): boolean {
  return user !== null && user.is_admin
}