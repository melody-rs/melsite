<script lang="ts">
  import { page } from "$app/state";
  let { href, name } = $props();

  function is_under_href(href: string): boolean {
    const paths = page.url.pathname.split("/");

    if (href == "/" && paths[1] == "home") return true;
    if ("/" + paths[1] == href) return true;
    return false;
  }
</script>

<a
  class="{is_under_href(href) ? 'active_item' : 'inactive_item'} nav_item"
  {href}>{name}</a
>

<style>
  .nav_item {
    font-family: "Iosevka SS05 Web", monospace;

    margin: 8px;
    font-size: min(6vw, 32px);
  }

  .inactive_item {
    color: #ffffff;
  }
  .active_item {
    color: #ff73f8;
  }

  .nav_item:hover {
    color: #843dff;
  }
</style>
