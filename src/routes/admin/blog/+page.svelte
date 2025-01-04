<script lang="ts">
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
  // TODO standardize + make function for
  let date_format = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
</script>

<svelte:head>
  <title>Blog</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <div style="padding-top:30px;"></div>
  <ul>
    {#each data.posts as post}
      <div class="post-container">
        <li>
          <a href={post.post_path} class="title">{post.metadata.title}</a>
          <p class="date">{date_format.format(new Date(post.metadata.date))}</p>
          <hr />
          <p>{post.metadata.description}</p>
        </li>
      </div>
    {/each}
  </ul>
</ContentWrapper>

<style>
  ul {
    display: grid;
    padding-left: 64px;
    padding-right: 64px;
    list-style-type: none;
    gap: 32px;
  }
  .post-container {
    background: rgba(20, 20, 20, 0.377);
    border: 0.5px solid rgba(64, 64, 64, 0.9);
    border-radius: 15px;
    padding: 12px 24px;
    box-shadow: 0 0 50px -10px rgba(40, 0, 77, 0.9);
  }
  .title {
    font-size: 32px;
  }
  .date {
    float: right;
    font-size: 32px;
    margin: 0;
  }

  p {
    font-size: 20px;
    font-weight: 300;
  }
</style>
