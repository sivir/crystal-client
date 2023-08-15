<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import { state} from "./lib";
	import type {Challenge} from "./lib";

	export let lockfile_exists: boolean;

	type Champion = {
		id: number,
		name: string,
		mastery_level: number,
		mastery_points: number,
		challenges?: { [key: number]: boolean }
	}

	type CommunityDragonChampion = {
		data: {
			[key: number]: {
				name: string,
				key: string
			}
		}
	}

	type MasteryData = {
		championId: number,
		championLevel: number,
		championPoints: number,
		chestGranted: boolean,
	}

	let champions: { [key: number]: Champion } = {};
	let table_data: Champion[] = [];
	let champion_dragon: CommunityDragonChampion;
	let mastery_data: MasteryData[];
	let table_challenges: Challenge[] = [];

	function refresh() {
		invoke("process_lockfile");
		invoke("get_champion_map").then(champion_data => {
			console.log("dragon", champion_data);
			champion_dragon = champion_data as CommunityDragonChampion;
		});
		invoke("update_all_data").then(() => {
			invoke("get_challenge_data").then(challenge_data => {
				invoke("get_champion_data").then(champion_data => {
					state.challenge_data = challenge_data as any;
					mastery_data = champion_data as MasteryData[];
				})
			})
		});
	}

	$: state && console.log(state);

	$: table_challenges = Object.values(state.challenge_data).filter(challenge => {
		return (challenge.idListType === "CHAMPION" && challenge.availableIds.length === 0);
	});

	$: table_challenges && console.log(table_challenges);

	$: if (lockfile_exists) {
		invoke("get_champion_map").then(champion_data => {
			console.log("dragon", champion_data);
			champion_dragon = champion_data as CommunityDragonChampion;
		});

		invoke("update_all_data").then(() => {
			invoke("get_challenge_data").then(challenge_data => {
				invoke("get_champion_data").then(champion_data => {
					state.challenge_data = challenge_data as any;
					mastery_data = champion_data as MasteryData[];
				})
			})
		});
	}

	$: if (champion_dragon) {
		console.log("champions");
		champions = Object.fromEntries(
			Object.entries(champion_dragon.data).map(([, value]) => [value.key, {
				id: parseInt(value.key),
				name: value.name,
				mastery_points: 0,
				mastery_level: 0
			}])
		);
	}

	$: table_data = Object.values(champions).sort((a, b) => {
		if (b.mastery_level === a.mastery_level)
			return b.mastery_points - a.mastery_points;
		return b.mastery_level - a.mastery_level;
	});

	$: if (Object.values(champions).length > 0 && mastery_data) {
		console.log(champions);
		mastery_data.forEach(x => {
			const id = x.championId;
			champions[id].id = id;
			champions[id].mastery_level = x.championLevel || 0;
			champions[id].mastery_points = x.championPoints || 0;
		});
	}
</script>

<main>
	<button on:click={refresh}>refresh</button>
	<div id="flow">
		west

		<table>
			<thead>
			<tr>
				<td>name</td>
				<td>mastery</td>
				<td>mastery points</td>
				{#each table_challenges as challenge}
					<td>
						<div title={challenge.description}>{challenge.name}</div>
					</td>
				{/each}
			</tr>
			</thead>
			{#each table_data as champion}
				<tr>
					<td>{champion.name}</td>
					<td>{champion.mastery_level}</td>
					<td>{champion.mastery_points}</td>
					{#each table_challenges as challenge}
						<td>
							{#if challenge.completedIds.includes(champion.id)}
								✅
							{:else}
								❌
							{/if}
						</td>
					{/each}
				</tr>
			{/each}
		</table>
	</div>
</main>

<style>
</style>