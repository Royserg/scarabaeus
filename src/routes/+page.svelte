<script lang="ts">
	import PluginButton from '$lib/components/plugin-button.svelte';
	import PluginLayout from '$lib/layout/plugin-layout.svelte';
	import TodayNote from '$lib/plugins/today-note.svelte';
	import { window } from '@tauri-apps/api';

	$effect(() => {
		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	});

	const handleKeyDown = (e: KeyboardEvent) => {
		const key = e.key;

		// Hide app when Esc is pressed
		if (key === 'Escape') {
			window.getCurrent().hide();
		}
	};
</script>

<PluginLayout>
	<ul
		class="flex w-full flex-col items-center justify-center gap-2 rounded-lg bg-white p-4 text-black"
	>
		<TodayNote />
		<PluginButton href="/git-commits">Git Commits</PluginButton>

		<!-- TODO: UUID generator -->

		<!-- There are some limitations on the custom command -->
		<!-- <Custom /> -->
	</ul>
</PluginLayout>

<style>
</style>
