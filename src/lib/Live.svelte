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

	type RiotData = {
		challenges: {
			value: number;
			challengeId: number;
		}[];
	}

	type TempPlayerGlobeChall = {
		[key: string]: RiotData["challenges"];
	}

	type TempGlobeDisplayEntry = {
		name: string;
		challenges: {
			[key: string]: number;
		}
	}

	let gamemode = "";
	let current_challenges: Challenge[] = [];
	let lobby_globes = [];
	let player_globe_challenges: TempPlayerGlobeChall = {};
	let globe_display: TempGlobeDisplayEntry[] = [];

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

	$: Promise.all(
		$state.lobby.map(x => 
			supabase.functions.invoke("get-user", {
				body: { riot_id: x },
			}).then(res => {
				const data: {riot_data: RiotData} = JSON.parse(res.data);
				return Object.entries(data.riot_data.challenges).filter(x => lobby_globes.includes(x[1].challengeId.toString())).map(x => x[1]);
			})
		)
	).then(results => {
		player_globe_challenges = results.reduce((acc, curr, index) => {
			acc[$state.lobby[index]] = curr;
			return acc;
		}, {});
	});

	$: globe_display = Object.entries(player_globe_challenges).map(x => {
		const entry: TempGlobeDisplayEntry = {
			name: x[0],
			challenges: {}
		};
		x[1].forEach(challenge => {
			entry.challenges[challenge.challengeId] = challenge.value;
		});
		return entry;
	});
</script>

<main>
	<!--<p>gamemode: {gamemode}</p>
	<p>state: {$state.phase}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select.map(x => $state.champion_names[x]))}</p>
	<p>other stuff: {JSON.stringify($state.table_challenges.filter(x => x.gameModes.includes("ARAM")).map(x => x.name))}</p>
	<p>pgc: {JSON.stringify(player_globe_challenges)}</p>-->
	<table>
		<tr>
			<td></td>
			{#each lobby_globes as globe}
				<td>{Object.entries($state.challenge_info).filter(x => x[0] === globe)[0][1].name}</td>
			{/each}
		</tr>
		{#each globe_display as player}
			<tr>
				<td>{player.name}</td>
				{#each lobby_globes as challenge}
					<td>{player.challenges[challenge] ?? 0}</td>
				{/each}
			</tr>
		{/each}
	</table>
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