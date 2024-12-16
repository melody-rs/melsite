<script>
  import { page } from "$app/stores";
  import Background from "$lib/aurora.svelte";
  import Navbar from "$lib/navbar.svelte";
  import Header from "$lib/header.svelte";
</script>

<svelte:head>
  <style>
    .actual-content {
      display: table-cell;
      vertical-align: middle;
    }

    h1 {
      text-align: center;
    }
  </style>
</svelte:head>

<Header />
<Background />
<Navbar />

<!-- yeah i dunno anymore lmao-->
<div style="display: table; margin:auto">
  <div class="actual-content">
    <h1>{$page.status} {$page.error?.message}</h1>
    <a href="https://http.cat">
      <img
        src="https://http.cat/{$page.status}"
        alt="A picture of a cat with the text {$page.status} 
  {$page.error?.message} underneath it"
      />
    </a>
  </div>
</div>
