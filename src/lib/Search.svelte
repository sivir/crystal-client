<script lang="ts">
	import { empty } from "svelte/internal";
import {supabase, state} from "./lib";
	let search: string = "";
	let temp: string = "";
	let data = {
		riot_data: {
			challenges: [],
		},
		lcu_data: {},
	};
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
		{#each Object.entries($state.challenge_data) as [_, challenge]}
			<div class="card">
				<div>{challenge.name}</div>
				{#each data.riot_data.challenges.filter(x => x.challengeId === challenge.id) as a}
					<div>{a.level}</div>
					<div>{a.value}/{JSON.stringify(challenge.thresholds[a.level])}</div>
				{/each}
				
			</div>
		{/each}
		<!-- <table>
			{#each Object.entries($state.challenge_data) as [_, challenge]}
				<tr>
					<td>{challenge.name}</td>
					{#each data.riot_data.challenges.filter(x => x.challengeId === challenge.id) as a}
						<td>{a.level}</td>
					{/each}
				</tr>
			{/each}
		</table>
		{temp} -->
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