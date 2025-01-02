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
    <data.content />
  </article>
</ContentWrapper>

<style>
  article {
    padding-left: 64px;
    padding-right: 64px;
  }
  .main-header {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 32px;
  }
  .date {
    float: right;
    font-size: 32px;
    margin: 0;
  }

  p {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 24px;
    font-weight: 300;
  }
</style>
