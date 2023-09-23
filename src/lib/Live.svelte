<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import {state} from "./lib";

	type GameflowSession = {
		gameData: {
			queue: {
				gameMode: string
			}
		}
	}

	let gamemode = "";

	$: if ($state.phase === "ChampSelect") {
		invoke("http_retry", {endpoint: "lol-gameflow/v1/session"}).then(x => {
			const session = x as GameflowSession;
			console.log("session", session);
			gamemode = session.gameData.queue.gameMode;
		});
	} else {
		gamemode = "";
	}
</script>

<main>
	<p>state: {$state.phase}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select)}</p>
	<p>current champ select names: {JSON.stringify(Object.values($state.champion_dragon.data).filter(champion => $state.champ_select.includes(parseInt(champion.key))).map(x => x.name))}</p>
	<p>other stuff:  </p>
</main>