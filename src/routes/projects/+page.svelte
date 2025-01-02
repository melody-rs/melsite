<script>
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import Skip from "$lib/a11y/skip.svelte";
  import SmallBlinkies from "$lib/components/small-blinkies.svelte";
</script>

<svelte:head>
  <title>Projects</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />

  <div class="container" id="main">
    <section class="project-section">
      <!-- TODO alt text -->
      <img class="project-img" src="/projects/fm.jpg" alt="" />
      <span>
        <h1>
          <img class="logo-svg" alt="" src="/logos/ruby.svg" />
          <a
            href="https://store.steampowered.com/app/1569440/OneShot_Fading_Memory/"
            >OneShot Fading Memory</a
          >
        </h1>
        Fading Memory was my first big project, I started it all the way back in
        2018. It's come a long way since then though, and I'm really proud of it!
        <br />
        While I don't have as much of a role as I used to, it's still one of my main
        projects!
      </span>
    </section>
    <section class="project-section">
      <!-- TODO alt text -->
      <span>
        <h1>
          <img class="logo-svg" alt="" src="/logos/rust.svg" /><a
            href="https://luminol.dev">Luminol</a
          >
        </h1>
        Luminol is another one of my big projects. Born out of frustration working
        with RPG Maker XP, Luminol is aiming to be a full replacement for the editor,
        a feat which none have achieved. It's still a work in progress but it's quite
        usable!
      </span>
      <img class="project-img" src="/projects/luminol.png" alt="" />
    </section>
    <section class="project-section">
      <!-- TODO alt text -->
      <img class="project-img" src="/projects/cloverlink.png" alt="" />
      <span>
        <h1>
          <img class="logo-svg" alt="" src="/logos/ruby.svg" />
          <a href="https://astrabit.itch.io/cloverlink">Cloverlink</a>
        </h1>
        Cloverlink is/was a multiplayer OneShot mod. I have a lot of fond memories
        working on it with my friends and messing around with multiplayer. It's kinda
        in a weird state right now. :( But! I might come back to it soon!
      </span>
    </section>
    <section class="project-section">
      <!-- TODO alt text -->
      <span>
        <h1><img class="logo-svg" alt="" src="/logos/c.svg" />Yuri Jam</h1>
        This was a project I was working on with a friend for the
        <a href="https://itch.io/jam/yuri-game-jam-2024">2024 Yuri Jam</a>
        that unfortunately didn't get finished. :( <br />
        I did quite a lot of the code for the project (I made the renderer, the scripting
        language, and a bunch of other stuff) but we didn't have any assets so the
        project hit a wall. I really wanna come back to it another day tho!
      </span>
      <img class="project-img" src="/projects/yuri-jam.png" alt="" />
    </section>
    <section class="project-section">
      <!-- TODO alt text -->
      <img class="project-img" src="/projects/mel-os.png" alt="" />
      <span>
        <h1><img class="logo-svg" alt="" src="/logos/rust.svg" />mel-os</h1>
        Funky little hobby OS that I'm working on. I've got some stuff implemented
        but nothing visually impressive
      </span>
    </section>
    <section class="project-section">
      <!-- TODO alt text -->
      <span>
        <h1>
          <img class="logo-svg" alt="" src="/logos/svelte.svg" />
          <a href="/">...This site!</a>
        </h1>
        This site is my first real taste of web dev. I've tried to make a site a
        couple of times, but either gave up or asked someone else to do it. Ngl,
        I'm enjoying it as much as I
        <i>hate</i> it.
      </span>
      <img class="project-img" src="/projects/melsite.png" alt="" />
    </section>
    <!-- bottom pad -->
    <div style="padding-bottom: 20px"></div>
  </div>
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
  .project-section {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    padding: 1rem;
  }

  .project-img {
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
    color: white;
    font-size: 18px;
    font-weight: 300;
    margin: 16px;
  }
</style>
