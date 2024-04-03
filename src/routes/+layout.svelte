<script lang="ts">
	import '../app.css';
	import { window } from '@tauri-apps/api';

	let { children } = $props();

	const hideWindow = () => {
		window.getCurrent().hide();
	};

	const handleBackdropClick = () => {
		hideWindow();
	};
</script>

<main class="h-screen w-screen">
	<div class="h-full w-full bg-white/20" onclick={handleBackdropClick}></div>

	{@render children()}
</main>
