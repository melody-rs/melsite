<script>
  import Background from "$lib/Background.svelte";
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
      /* Make it wrap */
      flex-wrap: wrap;
      /* Make it purple */
      background-color: rgba(128, 0, 196, 0.2);
      /* Add a border */
      border-top: 8px double rgba(39, 0, 60, 1);
    }

    .center-div-body {
      display: flex;
      /* Make it wrap on linebreaks */
      flex-wrap: wrap;
      flex-direction: column;
      /* Center it */
      align-items: center;
      justify-content: center;
    }
  </style>
</svelte:head>

<body>
  <Background />
  <div class="center-div-body">
    <h1>Mel's site!</h1>
    <br />
    <p>Add my site's blinkie!</p>
    <a href="https://melody-is.gay">
      <img
        src="melsite.gif"
        class="blinkie"
        alt="The text 'melody-is.gay' displayed on top of an aurora background. The '.gay' is glashing in rainbow colors."
      />
    </a>
    <pre>
      <code>
        &lta href="https://melody-is.gay"&gt
          &ltimg
            src="melsite.gif"
            class="blinkie"
          /&gt
        &lt/a&gt
      </code>
    </pre>
  </div>

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
        src="queer.png"
        class="blinkie"
        alt="The text 'you're telling me a queer coded this' displayed on top of a rainbow gradient"
      />
    </a>
    <a href="https://www.callistoAshley.dev" target="_blank">
      <img
        src="ashley.png"
        class="blinkie"
        alt="'callisto' written in a pixelated font against a background with pink stars"
      /></a
    >
    <img
      src="slugcat.png"
      class="blinkie"
      alt="A slugcat from rain world next to the text 'slug ca'"
    />
    <img
      src="archlinux.gif"
      class="blinkie"
      alt="The ArchLinux logo next to the text 'arch linux' on a marble background"
      title="I use Arch btw"
    />
    <img
      src="0235-rainbowstar.gif"
      class="blinkie"
      alt="The text 'SILLY!!' displayed next to a rainbow dancing star"
      title="SILLY!!!!!!!!!!!!!!!!"
    />
    <img
      src="0272-disabledpride.gif"
      class="blinkie"
      alt="The text 'Disabled Pride' surrounded by two disability pride flags"
      title="I might not fall under the disability pride label, but I think it's important to show support!"
    />
    <img
      src="blinkiesCafe-0r.gif"
      class="blinkie"
      alt="The text 'this user is trains' displayed on top of the transgender pride flag"
    />
    <img
      src="blinkiesCafe-44.gif"
      class="blinkie"
      alt="The text 'demiromantic pride' displayed on top of the demiromantic pride flag"
      title="Aro/Ace & Aroace pride!"
    />
    <img
      src="blinkiesCafe-OJ.gif"
      class="blinkie"
      alt="The text 'lesbian pride' displayed on top of the lesbian pride flag"
    />
  </footer>
</body>
