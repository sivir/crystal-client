<script lang="ts">
	import Greet from "./lib/Greet.svelte";
	import { watchImmediate } from "tauri-plugin-fs-watch-api";
	import { invoke } from "@tauri-apps/api/tauri"

	let lockfile_exists = false;
	let contents = [];

	invoke("read_file", {path:"C:\\Riot Games\\League of Legends\\lockfile"}).then(x => {
		lockfile_exists = true;
		console.log(x);
	})

	watchImmediate(
		"C:\\Riot Games\\League of Legends",
		(event) => {
			const { type, paths } = event;
			if (paths.includes("C:\\Riot Games\\League of Legends\\lockfile")) {
				console.log(event);
				if (typeof type != "string") {
					if ("create" in type) {
						lockfile_exists = true;
					} else if ("remove" in type) {
						lockfile_exists = false;
					}
				}
			}
		},
		{ recursive: true }
	);
</script>

<main class="container">
	<h1>Welcome to Tauri!</h1>

	<div class="row">
		<a href="https://vitejs.dev" target="_blank">
			<img src="/vite.svg" class="logo vite" alt="Vite Logo" />
		</a>
		<a href="https://tauri.app" target="_blank">
			<img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
		</a>
		<a href="https://svelte.dev" target="_blank">
			<img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
		</a>
	</div>

	<p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}</p>

	<div class="row">
		<Greet />
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
