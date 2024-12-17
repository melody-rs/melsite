<script>
  import SmallBlinkies from "./small-blinkies.svelte";
</script>

<footer>
  <SmallBlinkies />
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

    background-color: rgba(6, 6, 12, 0.9);
    border-top: 9px double rgba(85, 0, 120, 1);

    overflow-y: auto;
    max-height: 64px;
  }
</style>
