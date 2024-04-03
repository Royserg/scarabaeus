<script lang="ts">
	import PluginLayout from '$lib/layout/plugin-layout.svelte';
	import { window } from '@tauri-apps/api';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import { goto } from '$app/navigation';

	$effect(() => {
		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	});

	const handleKeyDown = (e: KeyboardEvent) => {
		const key = e.key;

		// Hide app when Esc is pressed
		if (key === 'Escape') {
			goto('/');
		}
	};

	const handleCommitTypeClick = async (content: string) => {
		await writeText(content);

		// Minimize window
		window.getCurrent().hide();
	};

	const commitTypes = [
		'feat: 🎸 ',
		'fix: 🐛 ',
		'chore: 🤖 ',
		'docs: 📖 ',
		'style: 🎨 ',
		'refactor: 💡 '
	];
</script>

<PluginLayout>
	<ul
		class="flex w-full flex-col items-center justify-center gap-2 rounded-lg bg-white p-4 text-black"
	>
		{#each commitTypes as type}
			<button
				onclick={() => handleCommitTypeClick(type)}
				class="w-full rounded-lg border border-black bg-white p-2 transition-colors duration-200 focus:bg-black focus:text-white"
			>
				{type}</button
			>
		{/each}
	</ul>
</PluginLayout>

<style>
</style>
