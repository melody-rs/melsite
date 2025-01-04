import { user_is_admin } from "$lib/auth";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

// We can count on this to block non-admins from viewing any of these pages
// ...But not from blocking any form requests
export const load: LayoutServerLoad = async ({ locals }) => {
  // deny request if user is not logged in or not an admin
  if (!user_is_admin(locals.user)) {
    error(403);
  }
  return {}
}