<script lang="ts">
	import Greet from "./lib/Greet.svelte";
	import {invoke} from "@tauri-apps/api/tauri"
	import {listen} from "@tauri-apps/api/event"
	import { appWindow } from '@tauri-apps/api/window'
	import {http_request} from "./lib/lib";
	import ChallengeTable from "./lib/ChallengeTable.svelte";
	import ChampionTable from "./lib/ChampionTable.svelte";

	let lockfile_exists = false;
	let gameflow = "None";

	let close, minimize, maximize;
	$: if (close) close.addEventListener("click", () => appWindow.hide());
	$: if (minimize) minimize.addEventListener("click", () => appWindow.minimize());
	$: if (maximize) maximize.addEventListener("click", () => appWindow.toggleMaximize());

	console.log("stuff");

	invoke("async_watch");
	listen("lockfile", x => {
		const payload = x.payload;
		console.log(payload);
		if (payload == "create" && lockfile_exists === false) {
			http_request("help", console.log);
			invoke("process_lockfile");
			invoke("start_lcu_websocket", {endpoints: ["OnJsonApiEvent_lol-gameflow_v1_gameflow-phase"]})
		}
		lockfile_exists = payload == "create";
	});
	listen("gameflow", x => {
		console.log(x);
		gameflow = x.payload.toString();
	})
	console.log("proc");
	invoke("process_lockfile");
</script>

<main class="container">
	<div data-tauri-drag-region class="titlebar">
		<div class="titlebar-button" id="minimize" bind:this={minimize}>
			<img
					src="https://api.iconify.design/mdi:window-minimize.svg"
					alt="minimize"
			/>
		</div>
		<div class="titlebar-button" id="maximize" bind:this={maximize}>
			<img
					src="https://api.iconify.design/mdi:window-maximize.svg"
					alt="maximize"
			/>
		</div>
		<div class="titlebar-button" id="close" bind:this={close}>
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</div>
	</div>

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

	<p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}. {gameflow}</p>

	<div class="row">
		<Greet/>
	</div>

	<ChampionTable />
	<ChallengeTable />
</main>

<style>
	.titlebar {
		height: 30px;
		background: #329ea3;
		user-select: none;
		display: flex;
		justify-content: flex-end;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
	}
	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 30px;
		height: 30px;
	}
	.titlebar-button:hover {
		background: #5bbec3;
	}

	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}
</style>
