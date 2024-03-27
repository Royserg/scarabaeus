<script lang="ts">
  import TodayNote from "./lib/today-note.svelte";
  import Custom from "./lib/custom.svelte";
  import GitCommit from "./lib/git-commit.svelte";
</script>

<main class="h-screen w-screen">
  <h1 class="pt-2 text-center text-black">Scarabs</h1>

  <div class="py-5"></div>

  <ul class="flex w-full flex-col items-center justify-center gap-4 text-black">
    <TodayNote />
    <GitCommit />

    <!-- There are some limitations on the custom command -->
    <!-- <Custom /> -->
  </ul>
</main>

<style>
</style>
