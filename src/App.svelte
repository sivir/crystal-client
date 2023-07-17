<script lang="ts">
	import Greet from "./lib/Greet.svelte";
	import {invoke} from "@tauri-apps/api/tauri"
	import {listen} from "@tauri-apps/api/event"

	let lockfile_exists = false;

	$: lockfile_exists ? () => {

	} : {}

	invoke("async_watch");
	listen("lockfile", x => {
		const payload = x.payload;
		console.log(payload);
		if (payload == "create" && lockfile_exists === false) {
			invoke("http_retry", {endpoint: "help"}).then(x => console.log(x));
			invoke("process_lockfile");
		}
		lockfile_exists = payload == "create";
	});
	console.log("proc");
	invoke("process_lockfile");
</script>

<main class="container">
	<h1>Welcome to Tauri!</h1>

	<div class="row">
		<a href="https://vitejs.dev" target="_blank">
			<img src="/vite.svg" class="logo vite" alt="Vite Logo"/>
		</a>
		<a href="https://tauri.app" target="_blank">
			<img src="/tauri.svg" class="logo tauri" alt="Tauri Logo"/>
		</a>
		<a href="https://svelte.dev" target="_blank">
			<img src="/svelte.svg" class="logo svelte" alt="Svelte Logo"/>
		</a>
	</div>

	<p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}</p>

	<div class="row">
		<Greet/>
	</div>
</main>

<style>
	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}
</style>
