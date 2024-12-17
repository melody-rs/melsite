<script lang="ts">
  let { children } = $props();
</script>

<div class="content-wrapper content-wrapper-outer">
  <div class="content-wrapper content-wrapper-inner">
    {@render children()}
  </div>
  <div class="border-fade"></div>
</div>
<div class="border-fade gradient-background"></div>

<style>
  .content-wrapper {
    max-width: 1000px;
    margin-right: auto;
    margin-left: auto;

    padding: 3px;
    padding-top: 0px;

    border-left: 3px solid rgba(85, 0, 120, 1);
    border-right: 3px solid rgba(85, 0, 120, 1);
  }
  .content-wrapper-outer {
    background-color: rgba(6, 6, 6, 0.9);
  }
  .content-wrapper-inner {
    padding: 10px;
    padding-top: 0px;
    overflow-y: auto;
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
