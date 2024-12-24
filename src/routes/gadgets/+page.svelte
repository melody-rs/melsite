<script>
  import ContentWrapper from "$lib/content-wrapper.svelte";
  import Navbar from "$lib/navbar.svelte";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Projects</title>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <div class="container">
    <a href="/gadgets/guestbook">Guestbook</a>
  </div>
</ContentWrapper>

<style>
</style>
