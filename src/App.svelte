<script lang="ts">
  // import Greet from "./lib/Greet.svelte";
  let count = $state(0);
  const increase = () => (count += 1);
  const decrease = () => (count -= 1);
</script>

<main class="h-screen w-screen">
  <h1 class="text-center text-black">Scarab</h1>

  <div class="py-10"></div>

  <div class="flex justify-center gap-4 text-black">
    <button onclick={increase}>+</button>
    <p>{count}</p>
    <button onclick={decrease}>-</button>
  </div>
</main>

<style>
</style>
