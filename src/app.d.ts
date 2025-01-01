
import type { Session, User } from "@prisma/client";

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}

    interface Locals {
      user: User | null;
      session: Session | null;
    }
  }
}

export { };
