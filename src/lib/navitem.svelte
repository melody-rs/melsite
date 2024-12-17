<script lang="ts">
  import { page } from "$app/state";
  let { href, name } = $props();
</script>

{#if href == page.url.pathname}
  <a class="nav_item" style="color: #ff73f8" {href}>{name}</a>
{:else}
  <a class="nav_item" style="color: #ffffff" {href}>{name}</a>
{/if}

<style>
  .nav_item {
    font-family: "Iosevka SS05 Web", monospace;

    margin: 8px;
    font-size: min(6vw, 32px);
  }

  .nav_item:hover {
    color: #843dff;
  }
</style>
