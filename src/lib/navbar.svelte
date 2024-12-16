<script lang="ts">
  import { onMount } from "svelte";

  onMount(() => {
    const navbar = <HTMLElement>document.getElementById("main_navbar");
    for (const element of navbar.children) {
      let link = <HTMLAnchorElement>element;
      if (link.href == document.URL) {
        link.classList.add("current_nav_item");
        link.classList.remove("base_nav_item");
      }
    }
  });
</script>

<!-- this makes it shrink to fit and centered? -->
<div style="display: table; margin:auto">
  <nav id="main_navbar">
    <a class="nav_item base_nav_item" href="/">Home</a>
    <a class="nav_item base_nav_item" href="/about">About</a>
    <a class="nav_item base_nav_item" href="/blog">Blog</a>
    <a class="nav_item base_nav_item" href="/gadgets">Gadgets</a>
    <a class="nav_item base_nav_item" href="/trinkets">Trinkets</a>
  </nav>
</div>

<style>
  nav {
    display: flex;
    justify-content: center;

    flex: 1 1 auto;

    margin-top: 4px;

    border: 32px solid;
    border-image: url("/border.png") 8 stretch;
    image-rendering: pixelated;
    background: rgba(3, 1, 4, 0.78);
    background-clip: content-box;
  }

  .nav_item {
    font-family: "Iosevka SS05 Web", monospace;

    margin: 8px;
    font-size: 32px;
  }
  .nav_item:hover {
    color: #843dff;
  }
  :global(.current_nav_item) {
    color: #52b4ff;
  }
  :global(.base_nav_item) {
    color: #ffffff;
  }
</style>
