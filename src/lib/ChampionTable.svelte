<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";

	export let lockfile_exists: boolean;

	type Champion = {
		name: string,
		mastery_level: number,
		mastery_points: number,
		challenges?: { [key: number]: boolean }
	}

	let champions: { [key: number]: Champion } = {};
	let champion_dragon: unknown;
	let mastery_data: unknown;

	$: if (lockfile_exists) {
		invoke("get_champion_map").then(champion_data => {
			champion_dragon = champion_data;
		});

		invoke("update_all_data").then(() => {
			invoke("get_challenge_data").then(challenge_data => {
				invoke("get_champion_data").then(champion_data => {
					console.log(champion_data);
					console.log(challenge_data);
					mastery_data = champion_data;
				})
			})
		});
	}

	$: if (champion_dragon) {
		champions = Object.fromEntries(
			Object.entries(champion_dragon.data).map(([, value]) => [value.key, {
				name: value.name,
				mastery_points: 0,
				mastery_level: 0
			}])
		);
	}

	$: if (mastery_data) {
		mastery_data.forEach(x => {
			const id = x.championId;
			champions[id].mastery_level = x.championLevel;
			champions[id].mastery_points = x.championPoints;
		});
	}
</script>

<main>
	west
	<div id="asdf">
		<!--
		<table>
			{#each Object.entries(champions) as [id, champion]}
				<tr>
					<td>{champion.name}</td>
					<td>{champion.mastery_level}</td>
					<td>{champion.mastery_points}</td>
				</tr>
			{/each}
		</table>-->
		{#each Object.entries(champions) as [id, champion]}
			<div>
				{champion.name}
				{champion.mastery_level}
				{champion.mastery_points}
			</div>
		{/each}
	</div>
</main>

<style>
	#asdf {
		overflow-y: auto;
	}
	main {
		box-sizing: border-box;
		max-height: inherit;
		display: flex;
		flex-direction: column;
	}
</style>