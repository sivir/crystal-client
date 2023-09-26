<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import {state} from "./lib";
	import type {Challenge, ChallengeData} from "./lib";

	type GameflowSession = {
		gameData: {
			queue: {
				gameMode: string
			}
		}
	}

	let gamemode = "";
	let current_challenges: Challenge[] = [];

	$: if ($state.phase === "ChampSelect") {
		invoke("http_retry", {endpoint: "lol-gameflow/v1/session"}).then(x => {
			const session = x as GameflowSession;
			console.log("session", session);
			gamemode = session.gameData.queue.gameMode;
			current_challenges = $state.table_challenges.filter(x => x.gameModes.includes(gamemode));
		});
	} else {
		gamemode = "";
	}
</script>

<main>
	<p>gamemode: {gamemode}</p>
	<p>state: {$state.phase}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select.map(x => $state.champion_names[x]))}</p>
	<p>other stuff: {JSON.stringify($state.table_challenges.filter(x => x.gameModes.includes("ARAM")).map(x => x.name))}</p>
	<table>
		{#each $state.champ_select as champion}
			<tr>
				<td>{$state.champion_names[champion]}</td>
				{#each current_challenges as challenge}
					<td>{challenge.completedIds.includes(champion)}</td>
				{/each}
			</tr>
		{/each}
	</table>
</main>