<script lang="ts">
  import { window } from "@tauri-apps/api";
  import GitCommit from "./lib/plugins/git-commit.svelte";
  import TodayNote from "./lib/plugins/today-note.svelte";
  // import { register } from "@tauri-apps/plugin-global-shortcut";

  $effect(() => {
    console.log("INIt");
    document.addEventListener("keydown", handleKeyDown);

    return () => document.removeEventListener("keydown", handleKeyDown);
  });

  const hideWindow = () => {
    window.getCurrent().hide();
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    const key = e.key;

    // Hide app when Esc is pressed
    if (key === "Escape") {
      hideWindow();
    }
  };

  const handleBackdropClick = () => {
    hideWindow();
  };
</script>

<main class="h-screen w-screen">
  <!-- Backdrop -->
  <!-- aria-roledescription="backdrop" -->
  <div class="h-full w-full bg-white/20" onclick={handleBackdropClick}></div>

  <section
    class="absolute left-[calc(50%-125px)] top-[20%] h-[300px] w-[250px]"
  >
    <ul
      class="flex w-[200px] flex-col items-center justify-center gap-2 rounded-lg bg-white p-4 text-black"
    >
      <TodayNote />
      <GitCommit />

      <!-- TODO: UUID generator -->

      <!-- There are some limitations on the custom command -->
      <!-- <Custom /> -->
    </ul>
  </section>
</main>

<style>
</style>
