import type { RequestEvent } from "@sveltejs/kit";

// sveltekit automatically sets secure for us in prod
export function set_session_cookie(event: RequestEvent, token: string, expires_at: Date) {
  event.cookies.set("session", token, {
    httpOnly: true,
    sameSite: "lax",
    expires: expires_at,
    path: "/"
  });
}
export function delete_session_cookie(event: RequestEvent) {
  event.cookies.set("session", "", {
    httpOnly: true,
    sameSite: "lax",
    maxAge: 0,
    path: "/"
  });
}