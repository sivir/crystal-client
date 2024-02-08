<script lang="ts">
	import {supabase, state} from "./lib";
	let search: string = "";
	let search2 = "";
	let temp2 = "";
	let temp: string = "";
	let data = {
		riot_data: {
			challenges: [],
		},
		lcu_data: {},
	};
	let data2 = {
		riot_data: {
			challenges: [],
		},
		lcu_data: {},
	};

	let mapdata1, mapdata2: {
		[key: string]: {
			value: number;
		}
	};

	$: if (temp !== "") {
		mapdata1 = data.riot_data.challenges.reduce((acc, x) => {
			acc[x.challengeId] = { value: x.value };
			return acc;
		}, {});
	}

	$: if (temp2 !== "") {
		mapdata2 = data2.riot_data.challenges.reduce((acc, x) => {
			acc[x.challengeId] = { value: x.value };
			return acc;
		}, {});
	}

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
			body: { riot_id: search },
		}).then(x => {
			temp = x.data;
			data = JSON.parse(x.data);
			console.log("get_user", data);
		});
	}} >search riot id</button>
	{#if temp !== ""}
		compare: <input bind:value={search2} />
		<button on:click={() => {
			supabase.functions.invoke("get-user", {
				body: { riot_id: search2 },
			}).then(x => {
				temp2 = x.data;
				data2 = JSON.parse(temp2);
				console.log("get_user2", data2);
			});
		}} >search riot id</button>
	{/if}
	{#if temp !== ""}
		<table>
			<tr>
				<th>name</th>
				<th>description</th>
				<th>value</th>
				{#if temp2 !== ""}
					<th>value2</th>
				{/if}
			</tr>
			{#each Object.entries($state.challenge_info) as a}
				{#if a[1].capstone === false}
					<tr>
						<td>{a[1].name}</td>
						<td>{a[1].description}</td>
						<td>{mapdata1[a[0]]?.value}</td>
						{#if temp2 !== ""}
							<td>{mapdata2[a[0]]?.value}</td>
						{/if}
					</tr>
				{/if}
			{/each}
		</table>
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