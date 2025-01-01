import { validate_session_token } from "$lib/auth";
import { delete_session_cookie, set_session_cookie } from "$lib/auth/cookies";
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }): Promise<Response> => {
  const token = event.cookies.get("session");
  if (token === undefined) {
    event.locals.user = null;
    event.locals.session = null;
    return resolve(event);
  }

  const { session, user } = await validate_session_token(token);
  if (session != null) {
    set_session_cookie(event, token, session.expires_at);
  } else {
    delete_session_cookie(event);
  }

  event.locals.session = session;
  event.locals.user = user;
  return resolve(event);
};