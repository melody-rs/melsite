<script>
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Blog</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <div class="center-div-body">
    <p>TODO!</p>
  </div>
</ContentWrapper>

<style>
  .center-div-body {
    display: flex;
    /* Make it wrap on linebreaks */
    flex-wrap: wrap;
    flex-direction: column;
    /* Center it */
    align-items: center;
    justify-content: center;
  }

  p {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 32px;
    font-weight: 300;
  }
</style>
