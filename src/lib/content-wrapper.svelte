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

    padding: 3px;
    padding-top: 0px;

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
  .content-wrapper-outer {
    background-color: rgba(6, 6, 6, 0.9);
  }
  .content-wrapper-inner {
    padding: 10px;
    padding-top: 0px;
    overflow-y: auto;
    height: calc(100vh - 200px);
    mask-image: linear-gradient(
      to bottom,
      #ffff 10px,
      #ffff calc(100% - 50px),
      #fff0 100%
    );
  }

  .border-fade {
    padding: 3px;

    /* Border image seems to be the only way to have a gradient color */
    border-image: linear-gradient(
        to bottom,
        rgba(85, 0, 120, 1) 0%,
        rgba(85, 0, 120, 0) 100%
      )
      1;
    border-width: 3px;
    border-top: 0px;
    border-bottom: 0px;

    border-style: solid;

    max-width: 1000px;
    height: 100px;
    margin-right: auto;
    margin-left: auto;
  }
  .gradient-background {
    background: linear-gradient(
      to bottom,
      rgba(6, 6, 6, 0.9) 0%,
      transparent 100%
    );
  }
</style>
