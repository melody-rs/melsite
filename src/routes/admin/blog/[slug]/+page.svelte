<script lang="ts">
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";

  let { data } = $props();
  let date_format = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
</script>

<svelte:head>
  <title>{data.title}</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <article>
    <h1 class="main-header">
      {data.title}
      <p class="date">{date_format.format(new Date(data.date))}</p>
    </h1>

    <hr />
    <div class="markdown">
      <data.content />
    </div>
  </article>
</ContentWrapper>

<style>
  article {
    padding-left: 64px;
    padding-right: 64px;
  }
  .main-header {
    font-size: 32px;
  }
  .date {
    float: right;
    font-size: 32px;
    margin: 0;
  }

  .markdown {
    :global(pre) {
      font-size: 14px;
      font-weight: 400;
      text-wrap: auto;
      border-radius: 15px;
      border: 2px solid rgb(54, 54, 54);
      background-color: rgb(26, 26, 26);
      box-shadow: 0 0 50px -10px rgba(40, 0, 77, 0.9);
      padding: 8px;
      tab-size: 2;
      max-inline-size: 100%;
    }
    :global(h1) {
      font-size: 32px;
    }
    :global(h2) {
      font-size: 28px;
    }
    :global(h3) {
      font-size: 24px;
    }
    :global(h4) {
      font-size: 20px;
    }
    :global(p) {
      font-size: 18px;
      font-weight: 400;
      line-height: 1.5;
    }
  }
</style>
