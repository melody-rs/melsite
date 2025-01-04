import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  try {
    const post = await import(`../drafts/${params.slug}.md`);
    const { title, date } = post.metadata;
    const content = post.default;

    return {
      content,
      title,
      date
    };
  }
  catch (e) {
    return error(404)
  }
};