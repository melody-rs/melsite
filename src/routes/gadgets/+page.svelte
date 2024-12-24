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
        <h1>
          <img class="logo-svg" alt="" src="/logos/svelte.svg" /><a
            href="/gadgets/guestbook">Guestbook</a
          >
        </h1>
        A little list of people who've visited this site!
      </span>
      <img class="gadget-img" src="/files/guestbook.png" alt="" />
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
    padding-top: 64px;
  }
  .gadget-section {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    padding: 1rem;
  }

  .gadget-img {
    height: 320px;
    /* Use transparent so the background image is visible */
    border: 5px solid transparent;
    padding: 2px;
    border-radius: 20px;
    background-image: linear-gradient(
      to right,
      rgb(190, 50, 255),
      rgb(89, 0, 255)
    );
    background-origin: border-box;
    background-clip: padding-box, border-box;
  }

  span {
    font-family: "Iosevka SS05 Web", monospace;
    color: white;
    font-size: 18px;
    font-weight: 300;
    margin: 16px;
  }
</style>
