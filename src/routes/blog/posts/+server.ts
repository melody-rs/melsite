import { json } from "@sveltejs/kit";
import type { RequestEvent } from "../../admin/$types";
import { fetch_markdown_posts } from "$lib/blog/posts";

export async function GET(event: RequestEvent): Promise<Response> {
  const posts = await fetch_markdown_posts();
  const sorted_posts = posts.sort((a, b) => {
    return new Date(b.metadata.date).getTime() - new Date(a.metadata.date).getTime();
  });
  return json(posts);
}