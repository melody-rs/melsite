import { fetch_markdown_posts, type MdPost } from '$lib/blog/posts';

const site_url = 'https://melody-is.gay';

export const prerender = true;

function render(posts: MdPost[]) {
  return `<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
<channel>
<title>Melody Is Gay</title>
<description>melody-rs's personal homepage</description>
<link>${site_url}</link>
<atom:link href="${site_url}/rss.xml" rel="self" type="application/rss+xml"/>
${posts
      .map(
        (post) => `<item>
<guid isPermaLink="true">${site_url}/${post.post_path}</guid>
<title>${post.metadata.title}</title>
<link>${site_url}/${post.post_path}</link>
<description>${post.metadata.title}</description>
<pubDate>${new Date(post.metadata.date).toUTCString()}</pubDate>
</item>`
      )
      .join('')}
</channel>
</rss>`
}

export const GET = async () => {
  const posts = await fetch_markdown_posts();
  const sorted_posts = posts.sort((a, b) => {
    return new Date(b.metadata.date).getTime() - new Date(a.metadata.date).getTime();
  });

  const body = render(sorted_posts);
  const options = {
    headers: {
      'Cache-Control': 'max-age=0, s-maxage=3600',
      'Content-Type': 'application/xml'
    }
  };

  return new Response(body, options);
};

