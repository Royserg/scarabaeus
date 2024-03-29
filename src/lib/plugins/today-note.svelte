<script lang="ts">
  import { path, window } from "@tauri-apps/api";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Command, open as shellOpen } from "@tauri-apps/plugin-shell";

  const padNumber = (num: number) => num.toString().padStart(2, "0");

  const LocalStorage = {
    selectedDir: "selectedDir",
  };

  const handleTodayClick = async () => {
    const today = new Date();
    const day = today.getDate();
    const month = today.getMonth() + 1;
    const year = today.getFullYear();

    const targetFilename = `${year}-${padNumber(month)}-${padNumber(day)}.md`;

    // Check if a directory has been selected
    let selectedDir = localStorage.getItem(LocalStorage.selectedDir);

    if (selectedDir === null) {
      try {
        selectedDir = await open({
          directory: true,
          defaultPath: await path.desktopDir(),
        });
      } catch (err) {
        console.log("Error selecting directory", err);
      }
    }

    // User cancelled the selection
    if (selectedDir === null) {
      return;
    }

    // Save selected directory
    localStorage.setItem(LocalStorage.selectedDir, selectedDir);

    const res = await Command.create("list-files", [selectedDir]).execute();
    const files = res.stdout.split("\n");
    const todayFilePath = `${selectedDir}/${targetFilename}`;

    if (!files.includes(targetFilename)) {
      try {
        await Command.create("create-file", [todayFilePath]).execute();
      } catch (err) {
        console.log("ERROR", err);
      }
    }

    try {
      await shellOpen(todayFilePath);

      // Hide app
      const currentWindow = window.getCurrent();
      await currentWindow.hide();
    } catch (err) {
      console.log("Failed opening file", err);
    }
  };
</script>

<button
  class="w-full rounded-lg border-2 border-black bg-white p-2 shadow-sm shadow-black"
  onclick={handleTodayClick}
>
  <h5>Today Note</h5>
</button>
