<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";

	export let lockfile_exists;

	type Champion = {
		name: string,
		mastery_level: number,
		mastery_points: number,
		challenges: { [key: number]: boolean }
	}

	let champions: Champion[] = [];

	$: if (lockfile_exists) {
		invoke("update_all_data").then(() => {
			invoke("get_challenge_data").then(challenge_data => {
				invoke("get_champion_data").then(champion_data => {
					console.log(champion_data);
					console.log(challenge_data);
					champions = champion_data.map(c => {
						return {
							name: c.championId,
							mastery_level: c.championLevel,
							mastery_points: c.championPoints,
							challenges: {}
						}
					})
				})
			})
		})
	}

</script>

<main>
	west
	<div>
		{#each champions as champion}
			<div>{champion.name}</div>
		{/each}
	</div>
</main>