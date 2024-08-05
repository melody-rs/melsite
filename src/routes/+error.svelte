<script>
  import { page } from "$app/stores";
</script>

<svelte:head>
  <style>
    html,
    body {
      height: 100%;
    }

    html {
      display: table;
      margin: auto;
    }

    body {
      display: table-cell;
      vertical-align: middle;
    }

    h1 {
      text-align: center;
    }
  </style>
</svelte:head>

<h1>{$page.status} {$page.error?.message}</h1>
<a href="https://http.cat">
  <img
    src="https://http.cat/{$page.status}"
    alt="A picture of a cat with the text {$page.status} 
  {$page.error?.message} underneath it"
  />
</a>
