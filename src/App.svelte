<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri"
	import {listen} from "@tauri-apps/api/event"
	import {appWindow} from '@tauri-apps/api/window'

	import ChallengeTable from "./lib/ChallengeTable.svelte";
	import ChampionTable from "./lib/ChampionTable.svelte";
	import Settings from "./lib/Settings.svelte";
	import Live from "./lib/Live.svelte";
	import Search from "./lib/Search.svelte";
	import Timeline from "./lib/Timeline.svelte";
	import Eternals from "./lib/Eternals.svelte";
	import {state,supabase, type ChallengeData} from "./lib/lib";

	type ChallengeDragon = {
		challenges: {
			[key: string]: {
				name: string;
				description: string;
				tags: {
					isCapstone?: string;
					isCategory?: string;
					parent?: string;
					priority?: number;
				};
				thresholds: {
					[key: string]: {
						value: number;
						threshold: number;
					}
				},
				seasons: [number];
			}
		},
		titles: {
			[key: string]: {
				name: string;
			}
		},
	}

	// different side panel pages
	enum Page {
		profile, // just a list of challenges
		champions, // champion table with mastery
		globe, // globetrotters + harmony list
		search, // search for another summoner, compare challenge progress
		live, // live lobby then champ select data
		eternals, // eternals progression and collection
		settings, // settings
		time // time sensitive challenge list (will probably have actual descriptions)
	}

	let lockfile_exists = false;
	let gameflow = "None";
	let page = Page.champions;

	console.log("stuff");

	$: if (lockfile_exists) {
		invoke("process_lockfile");
		invoke("start_lcu_websocket", {endpoints: ["OnJsonApiEvent_lol-champ-select_v1_session", "OnJsonApiEvent_lol-lobby_v2_lobby", "OnJsonApiEvent_lol-gameflow_v1_session"]});
		invoke("http_retry", {endpoint: "help"}).then(c => console.log("help", c));
		invoke("update_summoner_id").then(() => {
			invoke("get_riot_id").then(x => {
				$state.riot_id = x as string;
			});
		});

		invoke("update_all_data").then(() => 
			invoke("get_challenge_data").then(x => {
				$state.challenge_data = x as ChallengeData;
			})
		);

		invoke("get_champion_map2").then((x: {id: number, name: string}[]) => {
			$state.champion_names = Object.fromEntries(x.map(a => [a.id, a.name]));
		});

		invoke("get_challenges_map").then((x: ChallengeDragon) => {
			$state.challenge_info = Object.fromEntries(Object.entries(x.challenges).map(([key, value]) => [key, {
				name: value.name,
				description: value.description,
				capstone: value.tags.isCapstone ? value.tags.isCapstone === "Y" : false,
				parent: value.tags.parent ?? "",
				thresholds: value.thresholds ?? {"MASTER": {value: 0}},
				seasonal: value.seasons.length > 0,
			}]));
		});
		invoke("update_gameflow_phase");
		
		supabase.functions.invoke("challenge-info").then(x => console.log(JSON.parse(x.data)));

		invoke("async_watch");
	}
	
	type ChampSelect = {
		myTeam: {
			championId: number;
		}[];
		benchChampions: {
			championId: number;
		}[];
	}

	type Lobby = {
		summonerName: string;
		puuid: string;
	}

	listen("console_log", x => {
		console.log("console_log", x);
	});
	listen("lockfile", x => {
		const payload = x.payload;
		console.log(payload, lockfile_exists);
		if (payload === "create" && lockfile_exists === false) {
			lockfile_exists = true;
			console.log("lockfile exists");
		} else if (payload === "remove" && lockfile_exists === true) {
			lockfile_exists = false;
			console.log("lockfile doesn't exist");
		}
	});
	listen("gameflow", x => {
		console.log("phase", x);
		gameflow = x.payload.toString();
		$state.phase = gameflow;
		if (gameflow === "ChampSelect") {
			invoke("update_champ_select");
		} else if (gameflow === "None") {
			$state.lobby = [];
		}
	});
	listen("lobby", x => {
		console.log("lobby", x);
		const lobby = x.payload as Lobby[];
		$state.lobby = [];
		Promise.all(lobby.map(x => invoke("http_retry", {endpoint: "lol-summoner/v2/summoners/puuid/" + x.puuid}))).then((qwe: any) => {
			console.log(qwe);
			$state.lobby = qwe.map(x => x.gameName + "#" + x.tagLine);
			console.log("state_lobby", $state.lobby);
		});
	});
	listen("champ_select", x => {
		let champ_select = x.payload as ChampSelect;
		console.log("champ_select", x);
		console.log("myTeam", champ_select.myTeam);
		$state.champ_select = champ_select.myTeam.map(x => x.championId).concat(champ_select.benchChampions.map(x => x.championId));
	});
	invoke("process_lockfile");
</script>

<main>
	<!-- draggable titlebar -->
	<div data-tauri-drag-region class="titlebar">
		<div class="titlebar-button">crystal</div>
		<div>
			<button class="titlebar-button" on:click={appWindow.minimize}>
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path fill="#FFFFFF" d="M240-120v-80h480v80H240Z"/></svg>
			</button>
			<button class="titlebar-button" on:click={appWindow.toggleMaximize}>
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path fill="#FFFFFF" d="M160-760v-80h640v80H160Z"/></svg>
			</button>
			<button class="titlebar-button" on:click={appWindow.hide}>
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path fill="#FFFFFF" d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
			</button>
		</div>
	</div>

	<!-- main content -->
	<div id="sideways">
		<div id="sidebar">
			<div id="sidebuttons">
				<button on:click={() => page = Page.profile}>profile</button>
				<button on:click={() => page = Page.champions}>champions</button>
				<button on:click={() => page = Page.settings}>settings</button>
				<button on:click={() => page = Page.search}>search</button>
				<button on:click={() => page = Page.live}>live {#if $state.phase === "Lobby" || $state.phase === "ChampSelect"}🟢{/if}</button>
				<button on:click={() => page = Page.time}>time-sensitive</button>
				<button on:click={() => page = Page.eternals}>eternals</button>
			</div>
		</div>

		<div id="main">
			<p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}. {gameflow}</p>
			<div class="test" hidden={page !== Page.champions}>
				<ChampionTable {lockfile_exists}/>
			</div>
			<div class="test" hidden={page !== Page.profile}>
				<ChallengeTable/>
			</div>
			<div class="test" hidden={page !==Page.search}>
				<Search/>
			</div>
			<div class="test" hidden={page !== Page.settings}>
				<Settings/>
			</div>
			<div class="test" hidden={page !== Page.live}>
				<Live/>
			</div>
			<div class="test" hidden={page !== Page.time}>
				<Timeline/>
			</div>
			<div class="test" hidden={page !== Page.eternals}>
				<Eternals/>
			</div>
		</div>
	</div>
</main>

<style>
	.test {
		overflow: auto;
		margin: 20px;
	}

	#main {
		height: 100%;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	main {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
	}

	#sidebar {
		height: 100%;
		background: #212121;
		width: 150px;
	}

	#sidebuttons > button {
		padding: 10px;
	}

	#sidebuttons {
		display: flex;
		flex-direction: column;
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
		background: #1f1f1f;
		user-select: none;
		display: flex;
		justify-content: space-between;
	}

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		height: 30px;
		padding: 0;
	}

	.titlebar-button:hover {
		background: #5bbec3;
	}
</style>
