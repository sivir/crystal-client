<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri"
	import {listen} from "@tauri-apps/api/event"
	import { appWindow } from '@tauri-apps/api/window'
	import {http_request} from "./lib/lib";
	import ChallengeTable from "./lib/ChallengeTable.svelte";
	import ChampionTable from "./lib/ChampionTable.svelte";

	enum Page {
		champions, // champion table with mastery
		challenges, // just a list of challenges
		globe, // globetrotters + harmony list
		live, // live lobby then champ select data
	}

	let lockfile_exists = false;
	let gameflow = "None";
	let page = Page.champions;

	let close, minimize, maximize;
	$: if (close) close.addEventListener("click", () => appWindow.hide());
	$: if (minimize) minimize.addEventListener("click", () => appWindow.minimize());
	$: if (maximize) maximize.addEventListener("click", () => appWindow.toggleMaximize());

	console.log("stuff");

	$: lockfile_exists ? (() => {
		http_request("help", console.log);
		invoke("process_lockfile");
		invoke("start_lcu_websocket", {endpoints: ["OnJsonApiEvent_lol-gameflow_v1_gameflow-phase"]})
	})() : (() => {

	})();

	invoke("process_lockfile");
	invoke("async_watch");
	listen("lockfile", x => {
		const payload = x.payload;
		console.log(payload);
		if (payload == "create" && lockfile_exists === false) {
			lockfile_exists = true;
		} else if (payload == "remove" && lockfile_exists === true) {
			lockfile_exists = false;
		}
	});
	listen("gameflow", x => {
		console.log(x);
		gameflow = x.payload.toString();
	})
</script>

<main>
	<div data-tauri-drag-region class="titlebar">
		<div class="titlebar-button">crystal</div>

		<div>
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
		</div></div>
	</div>

	<div id="sideways">
		<div id="sidebar">
			<button on:click={() => page = Page.challenges}>challenges</button>
			<button on:click={() => page = Page.champions}>champions</button>
		</div>

		<div id="main">
			<p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}. {gameflow}</p>
			<div class="test" hidden={page !== Page.champions}>
				<ChampionTable {lockfile_exists}/>
			</div>
			<div class="test" hidden={page !== Page.challenges}>
				<ChallengeTable />
			</div>
		</div>
	</div>
</main>

<style>
	.test {
		height: 100%;
		overflow: auto;
	}

	#main {
		height: 100%;
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	main {
		height: 100%;
		width: 100%;
		background: red;
		display: flex;
		flex-direction: column;
	}

	#sidebar {
		min-height: 100%;
		background: #396cd8;
		width: 200px;
	}

	#sideways {
		flex: 1;
		overflow: auto;
		max-height: 100%;
		display: flex;
	}

	.titlebar {
		padding-left: 5px;
		padding-right: 5px;
		height: 30px;
		background: #329ea3;
		user-select: none;
		display: flex;
		justify-content: space-between;
	}

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		height: 30px;
	}

	.titlebar-button:hover {
		background: #5bbec3;
	}
</style>
