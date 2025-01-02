import { generateState } from "arctic";
import { github } from "$lib/auth/github";

import type { RequestEvent } from "@sveltejs/kit";

export async function GET(event: RequestEvent): Promise<Response> {
  // If already logged in, don't bother logging in again
  // hooks.server.ts takes care of validating the session and if its invalid will automatically reset the session for us
  if (event.locals.user !== null && event.locals.session !== null) {
    return new Response(null, {
      status: 302,
      headers: { Location: "/" }
    });
  }

  const state = generateState();
  const url = github.createAuthorizationURL(state, []);

  event.cookies.set("github_oauth_state", state, {
    path: "/",
    httpOnly: true,
    maxAge: 60 * 10,
    sameSite: "lax"
  });

  return new Response(null, {
    status: 302,
    headers: { Location: url.toString() }
  });
}