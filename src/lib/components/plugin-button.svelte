<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";

  let { children, onclick }: HTMLButtonAttributes = $props();

  type OnClick = MouseEvent & {
    currentTarget: EventTarget & HTMLButtonElement;
  };
  const handleClick = (e: OnClick) => {
    if (onclick) {
      onclick(e);
    }
  };
</script>

<button
  onclick={handleClick}
  class="w-full rounded-lg border-2 border-black bg-white p-2 shadow-sm shadow-black transition-colors duration-200 focus:bg-black focus:text-white"
>
  {#if children}
    {@render children()}
  {/if}
</button>
