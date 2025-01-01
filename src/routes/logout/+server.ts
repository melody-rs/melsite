import { invalidate_session } from "$lib/auth";
import { delete_session_cookie } from "$lib/auth/cookies";
import type { RequestEvent } from "@sveltejs/kit";

export async function GET(event: RequestEvent): Promise<Response> {
  if (event.locals.session === null) {
    return new Response(null, { status: 404 });
  }

  await invalidate_session(event.locals.session.id);
  delete_session_cookie(event);

  return new Response(null, {
    status: 302,
    headers: {
      Location: "/"
    }
  });
}