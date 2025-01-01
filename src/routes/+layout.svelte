<script lang="ts">
  let { children } = $props();

  import Aurora from "$lib/components/aurora.svelte";
  import Header from "$lib/components/header.svelte";
</script>

<Aurora />
<Header />

{@render children()}
