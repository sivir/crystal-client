<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import {state, supabase} from "./lib";
	import type {Challenge} from "./lib";

	type GameflowSession = {
		gameData: {
			queue: {
				gameMode: string
			}
		}
	}

	let gamemode = "";
	let current_challenges: Challenge[] = [];
	let lobby_globes = [];

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

	$: $state.lobby.map(x => {
		supabase.functions.invoke("get-user", {
			body: { summoner_name: x, riot_id: x },
		}).then(x => {
			const data = JSON.parse(x.data);
			lobby_globes.push(data.riot_data.challenges.filter(x => x.challengeId === 1000)[0].value);
		});
	});
</script>

<main>
	<p>gamemode: {gamemode}</p>
	<p>state: {$state.phase}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select.map(x => $state.champion_names[x]))}</p>
	<p>other stuff: {JSON.stringify($state.table_challenges.filter(x => x.gameModes.includes("ARAM")).map(x => x.name))}</p>
	<table>
		<tr>
			<td>Champion</td>
			{#each current_challenges as challenge}
				<td>{challenge.name}</td>
			{/each}
		</tr>
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