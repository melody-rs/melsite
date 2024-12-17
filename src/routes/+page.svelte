<script>
  import Background from "$lib/aurora.svelte";
  import Navbar from "$lib/navbar.svelte";
  import Header from "$lib/header.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Mel's site</title>
  <style>
    .blinkie {
      height: 31px;
      margin: 0 5px;
    }

    /* Move to bottom of page */
    footer {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;

      padding: 10px;

      display: flex;
      justify-content: center;
      flex-wrap: wrap;

      background-color: rgba(16, 0, 12, 0.9);
      border-top: 8px double rgba(85, 0, 120, 1);
    }

    .main-wrapper {
      max-width: 1000px;
      margin-right: auto;
      margin-left: auto;

      padding: 10px;
      padding-top: 0px;

      border-left: 8px solid rgba(85, 0, 120, 1);
      border-right: 8px solid rgba(85, 0, 120, 1);

      background-color: rgba(6, 6, 6, 0.9);
    }
    .gradient-wrapper {
      max-width: 1000px;
      height: 100px;
      margin-right: auto;
      margin-left: auto;

      background: linear-gradient(
        to bottom,
        rgba(6, 6, 6, 0.9) 0%,
        rgba(6, 6, 6, 0) 100%
      );
      padding: 10px;

      /* Border image seems to be the only way to have a gradient color */
      border-image: linear-gradient(
          to bottom,
          rgba(85, 0, 120, 1) 0%,
          rgba(85, 0, 120, 0) 100%
        )
        1;
      border-width: 8px;
      border-top: 0px;
      border-bottom: 0px;

      /* I wish I could make this a different style but it doesn't seem easy or possible to do */
      border-style: solid;
    }

    .content {
      margin-left: auto;
      margin-right: auto;
      padding-top: 10px;

      font-family: "Iosevka SS05 Web", monospace;
      font-size: 24px;
    }
    sup {
      font-size: 16px;
    }
  </style>
</svelte:head>

<Background />
<Header />

<div class="main-wrapper">
  <Navbar />

  <div class="content">
    <p>
      Hi! I'm Melody<sup>(she/her nya/nya's)</sup>, and this is my website! I'm
      a transfem aroace lesbian, who codes for fun and tries to makes games
      (sometimes). I've got lots of experience in low-level systems languages,
      especially Rust and C, and I'm sorta experienced at graphics programming!
    </p>
    <p>
      This place is meant to be a little site to introduce myself, host a little
      blog, and show off some neat little gadgets alongside explanations of how
      they work (like the aurora background!)
    </p>
    <p>
      The source code for my site isn't publically available yet, but when it is
      it'll be licensed under GPLv3 with exceptions noted. Some of the assets on
      this site aren't actually mine though, but they're all listed <a
        href="/assets">here</a
      >!
    </p>
    <p>
      Please <a href="/contact">reach out to me</a> if you have problems viewing
      this site, <i>especially</i> accessibility concerns, or if you just like a
      thing I've made!
    </p>
    <p>:3</p>
  </div>
</div>
<div class="gradient-wrapper"></div>

<footer>
  <a href="https://blinkies.cafe" target="_blank"
    ><img
      src="https://blinkies.cafe/b/display/blinkiesCafe-badge.gif"
      alt="blinkies.cafe | make your own blinkies!"
      class="blinkie"
    /></a
  >
  <a href="https://plasticdino.neocities.org/" target="_blank">
    <img
      src="/blinkies/queer.png"
      class="blinkie"
      alt="The text 'you're telling me a queer coded this' displayed on top of a rainbow gradient"
    />
  </a>
  <a href="https://www.callistoAshley.dev" target="_blank">
    <img
      src="/blinkies/ashley.png"
      class="blinkie"
      alt="'callisto' written in a pixelated font against a background with pink stars"
    /></a
  >
  <img
    src="/blinkies/slugcat.png"
    class="blinkie"
    alt="A slugcat from rain world next to the text 'slug ca'"
  />
  <img
    src="/blinkies/archlinux.gif"
    class="blinkie"
    alt="The ArchLinux logo next to the text 'arch linux' on a marble background"
    title="I use Arch btw"
  />
  <img
    src="/blinkies/0235-rainbowstar.gif"
    class="blinkie"
    alt="The text 'SILLY!!' displayed next to a rainbow dancing star"
    title="SILLY!!!!!!!!!!!!!!!!"
  />
  <img
    src="/blinkies/0272-disabledpride.gif"
    class="blinkie"
    alt="The text 'Disabled Pride' surrounded by two disability pride flags"
    title="I might not fall under the disability pride label, but I think it's important to show support!"
  />
  <img
    src="/blinkies/blinkiesCafe-0r.gif"
    class="blinkie"
    alt="The text 'this user is trains' displayed on top of the transgender pride flag"
  />
  <img
    src="/blinkies/blinkiesCafe-44.gif"
    class="blinkie"
    alt="The text 'demiromantic pride' displayed on top of the demiromantic pride flag"
    title="Aro/Ace & Aroace pride!"
  />
  <img
    src="/blinkies/blinkiesCafe-OJ.gif"
    class="blinkie"
    alt="The text 'lesbian pride' displayed on top of the lesbian pride flag"
  />
</footer>
