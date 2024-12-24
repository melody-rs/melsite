<script>
  import ContentWrapper from "$lib/content-wrapper.svelte";
  import Navbar from "$lib/navbar.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Trinkets</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <div class="center-div-body">
    <p>
      This page is <i>supposed</i> to have some little trinkets you can download
      and do whatever you want with, except I haven't made any yet :P
    </p>
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
    font-size: 24px;
    font-weight: 300;
  }
</style>
