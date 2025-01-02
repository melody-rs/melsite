<script>
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import Skip from "$lib/a11y/skip.svelte";
</script>

<svelte:head>
  <title>Trinkets</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />
  <div class="center-div-body" id="main">
    <p>
      This page is <i>supposed</i> to have some little trinkets you can download
      and do whatever you want with, except I haven't made any yet :P
    </p>
    <p>
      Site build status:
      <img
        src="https://github.com/melody-rs/melsite/actions/workflows/ci.yml/badge.svg"
        alt="A label showing the status of the site's CI"
        style="vertical-align:middle"
      />
    </p>
    <embed
      class="lang-stats"
      src="https://wakatime.com/share/@melody_rs/f5778ab6-7873-48eb-9269-3c731c9c25b5.svg"
    />
    <embed
      src="https://wakatime.com/share/@melody_rs/c9265dee-f1c5-4740-8c27-530044fbc5c6.svg"
    />
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

  .lang-stats {
    border-radius: 15px;
    max-width: 730px;
  }

  p {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 24px;
    font-weight: 300;
  }
</style>
