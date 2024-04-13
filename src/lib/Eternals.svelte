<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { state } from "./lib";

	let eternals_data = [];

	/*function update_eternals() {
		Promise.all(
			Object.keys($state.champion_names).map(champion_id => {
				return invoke("http_retry", {endpoint: `lol-statstones/v2/player-statstones-self/${champion_id}`}).then(x => {
					return x;
				});
			})
		).then(results => {
			console.log("wee", results);
			//eternals_data = results;
		});
	}
	
	// loop through all champions in list and get their eternals
	$: update_eternals();*/

	/*$: invoke("http_retry", {endpoint: "lol-statstones/v2/player-statstones-self/"}).then(x => {
		eternals_data = x;
	});*/

	$: invoke("http_retry", {endpoint: "lol-statstones/v2/player-summary-self"}).then(x => {
		console.log("wee", x);
		eternals_data = x;
	});
</script>

<main>
	{#if eternals_data.length > 0}
		<table>
			<tr>
				<th>name</th>
				<th>total milestones</th>
				<th>set 1</th>
				<th>set 2</th>
				<th>starter set</th>
			</tr>
			{#each eternals_data as a}
				<tr>
					<td>{$state.champion_names[a.championId]}</td>
					<td>{a.milestonesPassed}</td>
					{#each a.sets as set}
						<td>
							{#if set.stonesOwned == 0}
								🔒
							{:else}
								{set.milestonesPassed}
							{/if}
						</td>
					{/each}
				</tr>
			{/each}
		</table>
	{/if}
</main>
