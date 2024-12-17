<script>
  import Footer from "$lib/footer.svelte";
  import Navbar from "$lib/navbar.svelte";
</script>

<svelte:head>
  <title>Mel's site</title>
  <style>
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

<div class="content">
  <p>
    Hi! I'm Melody<sup>(she/her nya/nya's)</sup>, and this is my website! I'm a
    transfem aroace lesbian, who codes for fun and tries to makes games
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
    this site, <i>especially</i> accessibility concerns, or if you just like a thing
    I've made!
  </p>
  <p>:3</p>
</div>

<Footer />
