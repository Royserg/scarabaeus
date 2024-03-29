<script lang="ts">
  // NOTE:
  // Currently not functional.
  // Can't run arbirary shell comamnds
  //
  import { path } from "@tauri-apps/api";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Command, open as shellOpen } from "@tauri-apps/plugin-shell";

  const LocalStorage = {
    customCmd: "customCmd",
  };

  let customCmdVal = $state("");

  const handleSubmit = async () => {
    // let customCmd = localStorage.getItem(LocalStorage.customCmd);
    // Save selected directory
    // localStorage.setItem(LocalStorage.customCmd, <TODO>);

    if (customCmdVal.length === 0) {
      return;
    }

    console.log("customCmdVal", customCmdVal);
    try {
      const res = await Command.create("custom", [
        ...customCmdVal.split(" "),
      ]).execute();
      console.log({ res });

      // let result = await Command.create("exec-sh", [
      //   "-c",
      //   "echo 'Hello World!'",
      // ]).execute();
      // console.log(result);
    } catch (err) {
      console.log("Failed opening file", err);
    }
  };
</script>

<div class="flex w-[200px] flex-col gap-4 rounded-lg border-2 border-black p-2">
  <input
    bind:value={customCmdVal}
    class="w-full rounded-lg border border-black"
  />

  <button
    onclick={handleSubmit}
    class="w-full rounded-lg bg-slate-200 text-center transition-colors duration-300 hover:bg-slate-500 hover:text-white"
    >run</button
  >
</div>
