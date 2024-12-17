<script lang="ts">
  import Navitem from "./navitem.svelte";
</script>

<!-- this makes it shrink to fit and centered? -->
<div style="display: table; margin:auto">
  <nav id="main_navbar">
    <Navitem href="/" name="Home" />
    <Navitem href="/blog" name="Blog" />
    <Navitem href="/projects" name="Projects" />
    <Navitem href="/gadgets" name="Gadgets" />
    <Navitem href="/trinkets" name="Trinkets" />
  </nav>
</div>

<style>
  nav {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;

    flex: 1 1 auto;

    margin-top: 8px;

    border: min(6vw, 32px) solid;
    border-image: url("/border.png") 8 stretch;
    image-rendering: pixelated;
    background: rgba(3, 1, 4, 0.78);
    background-clip: content-box;
  }
</style>
