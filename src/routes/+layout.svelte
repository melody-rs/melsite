<script lang="ts">
  let { children } = $props();

  import Aurora from "$lib/aurora.svelte";
  import Header from "$lib/header.svelte";
</script>

<!-- <Aurora /> -->
<Header />

{@render children()}
