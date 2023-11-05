<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import {state, supabase} from "./lib";
	import type {CommunityDragonChampion, ChallengeData} from "./lib";

	export let lockfile_exists: boolean;

	type Champion = {
		id: number,
		name: string,
		mastery_level: number,
		mastery_points: number,
		chest_granted: boolean
	}
	type MasteryData = {
		championId: number,
		championLevel: number,
		championPoints: number,
		chestGranted: boolean,
	}

	let champions: { [key: number]: Champion } = {};
	let table_data: Champion[] = [];
	let mastery_data: MasteryData[];
	const default_sort = (a: Champion, b : Champion) => {
		if (b.mastery_level === a.mastery_level)
			return b.mastery_points - a.mastery_points;
		return b.mastery_level - a.mastery_level;
	};
	let table_sort = default_sort;

	let search = "";

	function refresh() {
		console.log("refresh called");
		//invoke("process_lockfile");
		invoke("update_all_data").then(() => {
			invoke("get_challenge_data").then(challenge_data => {
				invoke("get_champion_data").then(champion_data => {
					$state.challenge_data = challenge_data as ChallengeData;
					mastery_data = champion_data as MasteryData[];
					
					supabase.functions.invoke("update-user", {
						body: { riot_id: $state.riot_id, data: $state.challenge_data },
					});
				})
			})
		});
	}

	let state_phase = "None";
	$: state_phase = $state.phase;
	$: if (state_phase === "EndOfGame") {
		refresh();
	}

	$: $state.table_challenges = Object.values($state.challenge_data).filter(challenge => {
		if (challenge.category === "LEGACY") {
			return false;
		}
		const ignored_challenges = [401104, 401105, 501005, 501000, 501003]; // ignore m5, m7, eternals
		if (ignored_challenges.includes(challenge.id)) {
			return false;
		}
		return (challenge.idListType === "CHAMPION" && challenge.availableIds.length === 0);
	});

	$: $state.champion_names = Object.fromEntries(Object.entries($state.champion_dragon.data).map(([, value]) => [value.key, value.name]));

	$: if (lockfile_exists) {
		invoke("get_champion_map").then(champion_data => {
			console.log("dragon", champion_data);
			$state.champion_dragon = champion_data as CommunityDragonChampion;
		});

		refresh();
	}

	$: if ($state.champion_dragon) {
		champions = Object.fromEntries(Object.entries($state.champion_dragon.data).map(([, value]) => [value.key, {
			id: parseInt(value.key),
			name: value.name,
			mastery_points: 0,
			mastery_level: 0,
			chest_granted: false
		}]));
	}

	$: table_data = Object.values(champions).sort(table_sort);

	$: if (Object.values(champions).length > 0 && mastery_data) {
		console.log("champions", champions);
		mastery_data.forEach(x => {
			const id = x.championId;
			champions[id].id = id;
			champions[id].mastery_level = x.championLevel || 0;
			champions[id].mastery_points = x.championPoints || 0;
			champions[id].chest_granted = x.chestGranted || false;
		});
	}
</script>

<main>
	<div>
		<button on:click={refresh}>refresh</button>
		search: <input bind:value={search} />
		{search}
	</div>
	<div id="tab">
		<table>
			<thead>
			<tr>
				<td>name</td>
				<td>mastery</td>
				<td>mastery points</td>
				<td>chest</td>
				{#each $state.table_challenges as challenge}
					<td>
						<div title={challenge.description}><button on:click={() => table_sort = (a, b) => {
							const a_completed = challenge.completedIds.includes(a.id);
							const b_completed = challenge.completedIds.includes(b.id);
							if (a_completed === b_completed) {
								return default_sort(a, b);
							}
							return a_completed ? -1 : 1;
						}}>{challenge.name}</button></div>
					</td>
				{/each}
			</tr>
			</thead>
			{#each table_data as champion}
				{#if search === "" || champion.name.toLowerCase().includes(search.toLowerCase())}
					<tr>
						<td>{champion.name}</td>
						<td>{champion.mastery_level}</td>
						<td>{champion.mastery_points}</td>
						<td>{#if champion.chest_granted}✅{:else}❌{/if}</td>
						{#each $state.table_challenges as challenge}
							<td>
								{#if challenge.completedIds.includes(champion.id)}
									✅
								{:else}
									❌
								{/if}
							</td>
						{/each}
					</tr>
				{/if}
			{/each}
		</table>
	</div>
</main>

<style>
	#tab {
		height: 100%;
        overflow: auto;
	}

	main {
		height: 100%;
        flex: 1;
        display: flex;
        flex-direction: column;
	}

	table { 
		border-collapse: collapse; 
	}

	table > :global(:nth-child(151)) {
		border-bottom: 1px solid red;
	}
</style>