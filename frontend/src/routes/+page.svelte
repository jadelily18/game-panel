<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
	// let thing = await getThing();

	// async function getThing(): Promise<string> {
	// 	let fetch_res = fetch('localhost:8080')
	// 		.then((res) => res.text())
	// 		.then((data) => data);
	// 	return fetch_res;
	// }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<p>rand number: {data.number}</p>
