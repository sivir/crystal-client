<script lang="ts">
	import {supabase, state} from "./lib";
	let search: string = "";
	let temp: string = "";
	let data = {
		riot_data: {
			challenges: [],
		},
		lcu_data: {},
	};

	type TableChallenge = {
		id: string;
		name: string;
		description: string;
		value: number;
		// prev_threshold: number;
		// next_threshold: number;
		// percentile: number;
		// ranking: number;
	};

	let combined_data_1: TableChallenge[] = [];

	$: combined_data_1 = Object.values($state.challenge_data).map(challenge => {
		const data_value = data.riot_data.challenges.filter(x => x.challengeId === challenge.id)[0];
		const value = data_value?.value ?? 0;
		let entry = {
			id: challenge.id.toString(),
			name: challenge.name,
			description: challenge.description,
			value: value,
		};
		return entry;
	});
</script>

<main>
	search: <input bind:value={search} />
	<button on:click={() => {
		supabase.functions.invoke("get-user", {
			body: { summoner_name: search },
		}).then(x => {
			temp = x.data;
			data = JSON.parse(x.data);
			console.log("get_user", data);
		});
	}} >button</button>
	{#if temp !== ""}
		{#each combined_data_1 as challenge}
			<div class="card">
				<div>{challenge.name}</div>
				<div>{challenge.description}</div>
				<div>{challenge.value}</div>
			</div>
		{/each}
	{/if}
</main>

<style>
	.card {
		border: 1px solid #ccc;
		border-radius: 4px;
		margin: 4px;
		padding: 4px;
		display: flex;
		flex-direction: column;
	}
</style>