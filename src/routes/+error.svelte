<script>
  import { page } from "$app/stores";
</script>

<h1>{$page.status} {$page.error?.message}</h1>
<a href="https://http.cat">
  <img
    src="https://http.cat/{$page.status}"
    alt="A picture of a cat with the text {$page.status} 
  {$page.error?.message} underneath it"
  />
</a>
