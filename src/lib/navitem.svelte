<script lang="ts">
  import { page } from "$app/state";
  let { href, name } = $props();
</script>

{#if href == page.url.pathname}
  <a class="active_item nav_item" {href}>{name}</a>
{:else}
  <a class="inactive_item nav_item" {href}>{name}</a>
{/if}

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
