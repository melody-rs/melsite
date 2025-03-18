<script lang="ts">
  import Footer from "$lib/components/footer.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Skip from "$lib/a11y/skip.svelte";

  import type { PageData } from "./$types";
  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>Mel's site</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />

  <div class="content_flex">
    <div class="content inline_bordered" id="main">
      <p>
        Hi! I'm Melody<sup>(she/her nya/nya's)</sup>, and this is my website!
        I'm a transfem aroace lesbian, who codes for fun and tries to makes
        games (sometimes). I've got lots of experience in low-level systems
        languages, especially Rust and C, and I'm sorta experienced at graphics
        programming!
      </p>
      <p>
        This place is meant to be a little site to introduce myself, host a
        little blog, and show off some neat little gadgets alongside
        explanations of how they work (like the aurora background!)
      </p>
      <p>
        This site was designed on a 1440p display. I try my best to make it
        work, but if you're on mobile, here be dragons
      </p>
      <p>
        The source code for my site isn't publicly available yet, but when it is
        it'll be licensed under GPLv3 with exceptions noted. Some of the assets
        on this site aren't actually mine though, but they're all listed <a
          href="/home/external-assets">here</a
        >!
      </p>
      <p>
        Please <a href="/home/contact">reach out to me</a> if you have problems
        viewing this site, <i>especially</i> accessibility concerns, or if you just
        like a thing I've made!
      </p>
      <p>:3</p>
    </div>

    <aside class="right_sidebar inline_bordered">
      <p style="text-align: center;">Changelog</p>
      <div class="changelog_box">
        <p style="text-align: center;">Mar. 13, 2025</p>
        <ul>
          <li>Added changelog</li>
          <br />
          <li>Added visitor count</li>
          <br />
          <li>Refactored site layout a bit</li>
          <br />
          <li>More buttons! (They scroll now)</li>
        </ul>
      </div>
      <p style="position: absolute; bottom: 0">
        You are visitor number <b><i>{data.vistor_count}</i>!</b>
      </p>
    </aside>
  </div>

  <div class="webring-container">
    <script src="https://transring.neocities.org/ring.js"></script>
  </div>
</ContentWrapper>

<Footer />

<style>
  .webring-container {
    justify-content: center;
    text-align: center;
    display: flex;
    font-size: 24px;
    color: white;
    padding-top: 30px;
  }

  .content_flex {
    display: flex;
    padding-top: 10px;
  }

  .right_sidebar {
    /* no idea why just setting width: 25% doesn't work */
    min-width: 25%;
    max-width: 25%;
    margin-left: 10px;
    position: relative;
  }

  .content {
    margin-left: auto;
    margin-right: auto;

    font-size: 24px;
  }
  sup {
    font-size: 16px;
  }

  aside {
    font-size: 20px;
  }

  .inline_bordered {
    border: 2px solid rgb(149, 0, 207);
    border-radius: 8px;
    padding: 8px;
  }

  .changelog_box {
    background-color: rgb(0, 0, 0);
    border: 2px solid rgb(57, 0, 65);
    border-radius: 8px;
  }

  ul {
    color: white;
  }
</style>
