<script lang="ts">
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  const LocalStorage = {
    gitCommit: "gitCommit",
  };

  let dialog: HTMLDialogElement;

  const handleCommitsClick = async () => {
    console.log("DIALOG :", dialog);
    if (!dialog) {
      return;
    }

    dialog.showModal();
    // Save selected directory
    // localStorage.setItem(LocalStorage.selectedDir, selectedDir);
  };

  const handleCommitTypeClick = async (content: string) => {
    console.log("SAVING TYPE", content);
    await writeText(content);
  };

  const commitTypes = [
    "feat: 🎸",
    "fix: 🐛",
    "chore: 🤖",
    "docs: 📖",
    "style: 🎨",
    "refactor: 💡",
  ];
</script>

<button
  class="w-[200px] rounded-lg border-2 border-black p-2"
  onclick={handleCommitsClick}
>
  <h5>Git Commits</h5>
</button>

<dialog
  bind:this={dialog}
  class="h-full w-full rounded-lg p-2 backdrop:bg-gray-50"
>
  <div class="py-4"></div>

  <div class="flex w-full flex-col items-start gap-1">
    {#each commitTypes as type}
      <button
        onclick={() => handleCommitTypeClick(type)}
        class="w-full rounded-lg border border-black p-2 transition-colors duration-200 focus:bg-black focus:text-white"
      >
        {type}</button
      >
    {/each}
  </div>
</dialog>
