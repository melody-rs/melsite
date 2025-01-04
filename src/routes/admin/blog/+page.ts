import { fetch_draft_posts } from "$lib/blog/drafts";
import type { PageLoad } from "./[slug]/$types";

export const load: PageLoad = async (event) => {
  const posts = await fetch_draft_posts();
  const sorted_posts = posts.sort((a, b) => {
    return new Date(b.metadata.date).getTime() - new Date(a.metadata.date).getTime();
  });
  return { posts: sorted_posts };
}