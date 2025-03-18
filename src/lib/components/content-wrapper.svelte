<script lang="ts">
  let { children } = $props();
</script>

<div class="content-wrapper">
  {@render children()}
</div>

<style>
  .content-wrapper {
    max-width: 1000px;
    margin-right: auto;
    margin-left: auto;

    border-left: 9px double rgba(85, 0, 120, 1);
    border-right: 9px double rgba(85, 0, 120, 1);

    background-color: rgba(6, 6, 6, 0.9);
    padding: 10px;
    padding-top: 0px;
    overflow-y: auto;
    height: calc(100vh - 100px);
    mask-image: linear-gradient(
      to bottom,
      #ffff 10px,
      #ffff calc(100% - 50px),
      #fff0 100%
    );
  }
</style>
