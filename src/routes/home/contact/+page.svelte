<script>
  import ContentWrapper from "$lib/content-wrapper.svelte";
  import Navbar from "$lib/navbar.svelte";
  import Skip from "$lib/skip.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Contact</title>
  <style>
    .center-div-body {
      display: flex;
      /* Make it wrap on linebreaks */
      flex-wrap: wrap;
      flex-direction: column;
      /* Center it */
      align-items: center;
      justify-content: center;
    }

    p {
      font-family: "Iosevka SS05 Web", monospace;
      font-size: 32px;
      font-weight: 300;
      margin: 16px;
    }
  </style>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />
  <div class="center-div-body" id="main">
    <p style="margin-top: 48px;">
      email: <a href="mailto:melody@melody-is.gay">melody@melody-is.gay</a>
    </p>
    <p style="font-size: 20px; margin-top: 0px;">
      If possible, use my <a href="/files/pgp.asc">pgp key</a> [6D4A 6445 B602 1B9D
      1A53 439D DD36 D4EA F490 1935]
    </p>
    <p>
      secondary email: <a href="mailto:melody@nowaffles.com"
        >melody@nowaffles.com</a
      >
    </p>
    <p style="font-size: 20px; margin-top: 0px;">
      (I may be slow to respond if you email this address)
    </p>

    <p>
      github: <a href="https://github.com/melody-rs">melody-rs</a>
    </p>
  </div>
</ContentWrapper>
