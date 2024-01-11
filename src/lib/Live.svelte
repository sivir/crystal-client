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
	let player_globe_challenges = {};

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

	$: lobby_globes = Object.entries($state.challenge_info).filter(x => x[1].parent === "303500").map(x => x[0]);

	$: $state.lobby.map(x => {
		supabase.functions.invoke("get-user", {
			body: { riot_id: x },
		}).then(res => {
			const data = JSON.parse(res.data);
			player_globe_challenges[x] = Object.entries(data.riot_data.challenges).filter((x: any) => lobby_globes.includes(x.challengeId as string)).map(x => x[1]);
		});
	});
</script>

<main>
	<p>gamemode: {gamemode}</p>
	<p>state: {$state.phase}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select.map(x => $state.champion_names[x]))}</p>
	<p>other stuff: {JSON.stringify($state.table_challenges.filter(x => x.gameModes.includes("ARAM")).map(x => x.name))}</p>
	<p>lbyg: {JSON.stringify(lobby_globes)}</p>
	<p>pgc: {JSON.stringify(player_globe_challenges)}</p>
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