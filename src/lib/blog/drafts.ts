export interface MdPost {
  metadata: Record<string, string>,
  post_path: string,
}
export async function fetch_draft_posts(): Promise<MdPost[]> {
  const post_files = import.meta.glob("/src/routes/admin/blog/drafts/*.md");
  const iter = Object.entries(post_files).map(async ([path, resolver]) => {
    const { metadata } = <any>await resolver();
    const post_path = `/admin/blog/${path.slice(30, -3)}`;
    return { metadata, post_path }
  });
  return await Promise.all(iter);
}
