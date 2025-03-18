<script>
  import Allblinkies from "./allblinkies.svelte";
</script>

<footer>
  <div class="scrolling">
    <!-- we need two to make the scrolling seamless (i.e it wraps) -->
    <Allblinkies />
    <Allblinkies />
  </div>
</footer>

<style>
  .scrolling {
    animation: scrolling_anim 30s linear infinite;
    white-space: nowrap;
  }
  @keyframes scrolling_anim {
    0% {
      transform: translateX(0);
    }
    100% {
      transform: translateX(-50%);
    }
  }
  footer:hover .scrolling {
    animation-play-state: paused;
  }

  /* Move to bottom of page */
  footer {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;

    padding: 10px;

    display: flex;

    background-color: rgba(6, 6, 12, 0.9);
    border-top: 9px double rgba(85, 0, 120, 1);

    max-height: 64px;
  }
</style>
