<script>
  import SmallBlinkies from "$lib/small-blinkies.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Assets</title>
</svelte:head>

<div class="container">
  <section class="asset-section">
    <img
      class="asset-img"
      src="/aurora/0124-stars.gif"
      alt="an old 90s style gif of stars"
    />
    <span>
      Star background <br />
      <a href="https://blinkies.cafe/archive">blinkies.cafe</a>
    </span>
  </section>
  <section class="asset-section">
    <span>
      Aurora shader <br />
      <a href="https://www.shadertoy.com/view/XtGGRt">shadertoy.com</a>
    </span>
    <img
      class="asset-img"
      src="/credits/aurora.png"
      alt="a render of an aurora"
    />
  </section>
  <section class="asset-section">
    <div class="blinkie-container">
      <SmallBlinkies />
    </div>
    <span style="font-size: 18px">
      Blinkies <br />
      <a href="https://blinkies.cafe">blinkies.cafe</a> <br />
      callisto blinkie <br />
      <a href="https://callistoAshley.dev">callistoAshley.dev</a> <br />
      queer coded & nap time blinkie <br />
      <a href="https://plasticdino.neocities.org/">
        plasticdino.neocities.org</a
      >
    </span>
  </section>
  <section class="asset-section">
    <span>
      Iosevka SS05 <br />
      <a href="https://typeof.net/Iosevka/">typeof.net</a>
    </span>
    <!-- TODO alt text for this -->
    <img class="asset-img" src="/credits/iosevka.png" alt="" />
  </section>
  <section class="asset-section">
    <img
      class="asset-img"
      src="/favicon.png"
      style="image-rendering: pixelated; size: 120px;"
      alt=""
    />
    <span>
      Trans heart <br />
      <a
        href="https://discord.com/channels/403698615446536203/586655711564529687/901636858742706188"
        >Celeste discord</a
      >
    </span>
  </section>
</div>

<style>
  .container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    padding-top: 64px;
  }
  .asset-section {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    padding: 1rem;
  }

  .blinkie-container {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    width: 300px;
    row-gap: 4px;
  }

  .asset-img {
    height: 120px;
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
