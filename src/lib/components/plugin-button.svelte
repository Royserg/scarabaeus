<script lang="ts">
	import type { HTMLAnchorAttributes } from 'svelte/elements';

	let { children, href, onclick }: HTMLAnchorAttributes = $props();
</script>

<a
	{href}
	{onclick}
	class="w-full rounded-lg border-2 border-black bg-white p-2 text-center shadow-sm shadow-black transition-colors duration-200 focus:bg-black focus:text-white"
>
	{#if children}
		{@render children()}
	{/if}
</a>
