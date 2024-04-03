<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
		class?: string;
	};
	const { children }: Props = $props();
</script>

<!-- TODO: add cn util and add the class passed in props -->
<section class="absolute left-[calc(50%-125px)] top-[20%] h-[300px] w-[250px]">
	{#if children}
		{@render children()}
	{/if}
</section>
