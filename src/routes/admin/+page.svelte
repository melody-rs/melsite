<script>
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import Skip from "$lib/a11y/skip.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Trinkets</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />
  <div class="center-div-body" id="main">
    <p>
      Hello! If you somehow got to this page <i>and you're not an admin</i> please
      tell me!
    </p>
    <p>
      Anyway, here's some dashboard stuff to manage db things (I trust you know
      what you're doing!)
    </p>
    <a href="/admin/users">User management</a>
    <a href="/admin/guestbook">Guestbook</a>
  </div>
</ContentWrapper>

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

  a {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 32px;
    color: #52b4ff;
  }

  a:hover {
    color: #843dff;
  }

  p {
    font-family: "Iosevka SS05 Web", monospace;
    font-size: 24px;
    font-weight: 300;
  }
</style>
