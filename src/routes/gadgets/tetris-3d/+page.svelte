<script lang="ts">
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import { onMount } from "svelte";
  const module = import("tetris-3d");

  if (import.meta.hot) {
    import.meta.hot.invalidate();
  }

  onMount(() => {
    module.then((module) => {
      const spinner = document.getElementById("spinner");
      if (spinner) spinner.style.display = "none";

      const canvas = document.getElementById("bevy-canvas");
      if (canvas) canvas.style.display = "block";

      module.run();
    });
  });
</script>

<ContentWrapper>
  <Navbar />

  <div class="game-container">
    <canvas class="bevy-canvas" id="bevy-canvas" style="display:none;"></canvas>
  </div>

  <div class="spinner" id="spinner"></div>
</ContentWrapper>

<style>
  .spinner {
    position: absolute;
    top: 50%;
    left: 50%;

    border: 4px solid #f3f3f3;
    border-top: 4px solid rgb(152, 52, 219);
    border-radius: 50%;
    width: 32px;
    height: 32px;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .game-container {
    margin-top: 8px;
    display: flex;
    justify-content: center;
  }
  .bevy-canvas {
    border: 4px double #730ba3;
    border-radius: 4px;
  }
</style>
