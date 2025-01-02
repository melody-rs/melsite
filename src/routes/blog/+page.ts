import { fetch_markdown_posts } from "$lib/blog/posts";
import type { PageLoad } from "./[slug]/$types";

export const load: PageLoad = async (event) => {
  const posts = await fetch_markdown_posts();
  const sorted_posts = posts.sort((a, b) => {
    return new Date(b.metadata.date).getTime() - new Date(a.metadata.date).getTime();
  });
  return { posts: sorted_posts };
}