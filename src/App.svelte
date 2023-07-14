<script lang="ts">
	import Greet from "./lib/Greet.svelte";
	import {invoke} from "@tauri-apps/api/tauri"
	import {listen} from "@tauri-apps/api/event"

	let lockfile_exists = false;
	let contents = [];
	let port, prefix, auth;

	$: lockfile_exists ? () => {

	} : {}

	function http_request(url: string) {
		return invoke("http_request", {url: url, auth: auth});
	}

	function lockfile_read() {
		invoke("process_lockfile");
		invoke("read_file", {path: "C:\\Riot Games\\League of Legends\\lockfile"}).then(x => {
			lockfile_exists = true;
			contents = x.split(":");
			//console.log(contents);

			port = contents[2];
			const password = contents[3];
			prefix = contents[4];

			auth = btoa(`riot:${password}`);

			//invoke("http_request", {url: `https://127.0.0.1:${port}/help`, auth: auth}).then(x => console.log(x));
			//console.log(
				http_request(`https://127.0.0.1:${port}/help`).then(x => console.log(x));
				invoke("start_lcu_websocket", {port: `wss://127.0.0.1:${port}/`, auth: `Basic ${auth}`})
			//);
		});
	}

	invoke("async_watch");
	listen("lockfile", x => {
		const payload = x.payload;
		console.log(payload);
		if (payload == "create") {
			invoke("process_lockfile");
		}
	});
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
