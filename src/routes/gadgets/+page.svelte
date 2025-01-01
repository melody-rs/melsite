<script>
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import Skip from "$lib/a11y/skip.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Gadgets</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />
  <div class="container" id="main">
    <div style="padding-top:30px"></div>
    <section class="gadget-section">
      <span>
        Gadgets are random things that run in your browser, alongside an
        accompanying blog post (if it's not too simple). Gadgets are designed
        primarily to show off what I'm capable of coding, as well as to be fun
        little things you can mess around with. At the moment, I'm working on a
        fluid sim, but I'm hoping to put some bevy games up here!

        <i>If</i> I can somehow run an emulator I'm hoping to put my hobby OS on
        here too!
      </span>
    </section>

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
    padding-left: 1rem;
    padding-right: 1rem;
  }

  h1 {
    margin-top: 0;
  }

  span {
    font-family: "Iosevka SS05 Web", monospace;
    color: white;
    font-size: 18px;
    font-weight: 300;
    margin-bottom: 16px;
  }
</style>
