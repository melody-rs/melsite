<script>
  import ContentWrapper from "$lib/content-wrapper.svelte";
  import Navbar from "$lib/navbar.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Projects</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <div class="container">
    <section class="gadget-section">
      <span>
        <h1 style="text-align: center;">
          <img class="logo-svg" alt="" src="/logos/svelte.svg" />
          <a href="/gadgets/guestbook">Guestbook</a>
        </h1>
        A little list of people who've visited this site! It's nothing special and
        is written entirely in Svelte/Typescript. It's one of the first things I
        tackled after laying out most of the site, primarily to learn how to send
        stuff between the client and server, as well as perform basic validation/filtering.
      </span>
    </section>
  </div>
  <div style="padding-bottom: 20px"></div>
</ContentWrapper>

<style>
  .logo-svg {
    height: 32px;
    filter: invert(100%);
    vertical-align: middle;
    padding-right: 16px;
  }

  .container {
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .gadget-section {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    padding: 1rem;
  }

  span {
    font-family: "Iosevka SS05 Web", monospace;
    color: white;
    font-size: 18px;
    font-weight: 300;
    margin-bottom: 16px;
  }
</style>
