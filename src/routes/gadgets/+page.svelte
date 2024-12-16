<script>
  import Background from "$lib/aurora.svelte";
  import Navbar from "$lib/navbar.svelte";
  import Header from "$lib/header.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>About</title>
  <style>
    .blinkie {
      height: 31px;
      margin: 0 5px;
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

    p {
      font-family: "Iosevka SS05 Web", monospace;
      font-size: 32px;
      font-weight: 300;
    }
  </style>
</svelte:head>

<Background />
<Header />

<Navbar />

<div class="center-div-body">
  <p>TODO!</p>
</div>
