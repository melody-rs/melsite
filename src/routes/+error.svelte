<script>
  import { page } from "$app/state";
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
</script>

<svelte:head>
  <style>
    .actual-content {
      display: table-cell;
      vertical-align: middle;
    }

    h1 {
      font-family: "Iosevka SS05 Web", monospace;
      text-align: center;
    }
  </style>
</svelte:head>

<ContentWrapper>
  <Navbar />
  <!-- yeah i dunno anymore lmao-->
  <div style="display: table; margin:auto">
    <div class="actual-content">
      <h1>{page.status} {page.error?.message}</h1>
      <a href="https://http.cat">
        <img
          src="https://http.cat/{page.status}"
          alt="A picture of a cat with the text {page.status} 
  {page.error?.message} underneath it"
        />
      </a>
    </div>
  </div>
</ContentWrapper>
