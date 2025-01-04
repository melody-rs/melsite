import { user_is_admin } from "$lib/auth";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  // deny request if user is not logged in or not an admin
  if (!user_is_admin(locals.user)) {
    error(403);
  }
  return {}
}