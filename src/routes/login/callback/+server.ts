import { generate_session_token, create_session } from "$lib/auth";
import { set_session_cookie } from "$lib/auth/cookies";
import { github } from "$lib/auth/github";
import prisma from "$lib/prisma";

import type { RequestEvent } from "@sveltejs/kit";
import type { OAuth2Tokens } from "arctic";

export async function GET(event: RequestEvent): Promise<Response> {
  const code = event.url.searchParams.get("code");
  const state = event.url.searchParams.get("state");
  const stored_state = event.cookies.get("github_oauth_state");

  if (code === null || state === null || stored_state === undefined) {
    return new Response(null, { status: 400 });
  }

  if (state !== stored_state) {
    return new Response(null, { status: 400 });
  }

  let tokens: OAuth2Tokens;
  try {
    tokens = await github.validateAuthorizationCode(code);
  } catch (e) {
    // Invalid code/client credentials
    return new Response(null, { status: 400 });
  }

  const github_user_response = await fetch("https://api.github.com/user", {
    headers: {
      Authorization: `Bearer ${tokens.accessToken()}`
    }
  });
  const github_user = await github_user_response.json();
  const github_user_id = <number>github_user.id;
  const github_username = github_user.login;

  // no idea why id needs to be undefined
  const existing_user = await prisma.user.findUnique({
    where: { github_id: github_user_id }
  });

  if (existing_user) {
    const session_token = generate_session_token();
    const session = await create_session(session_token, existing_user.id);
    set_session_cookie(event, session_token, session.expires_at);

    return new Response(null, {
      status: 302,
      headers: {
        Location: "/"
      }
    });
  }

  const user = await prisma.user.create({
    data: { username: github_username, github_id: github_user_id }
  });
  const session_token = generate_session_token();
  const session = await create_session(session_token, user.id);
  set_session_cookie(event, session_token, session.expires_at);

  return new Response(null, {
    status: 302,
    headers: {
      Location: "/"
    }
  });
}