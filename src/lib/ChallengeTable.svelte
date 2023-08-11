<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";

	type Challenge = {
		name: string,
		points: number
	}

	let challenges: unknown = {};

	invoke("update_all_data").then(() => {
		invoke("get_challenge_data").then(x => {
			challenges = x;
		})
	});
</script>

<main>
	whee
	<div id="chal" style="width: 90%; height: 100px">
		<table >
			{#each Object.entries(challenges) as [id, challenge]}
				<tr>
					<td>{challenge.name}</td>
					<td>{challenge.currentLevel}</td>
				</tr>
			{/each}
		</table>
	</div>
</main>

<style>
	main {
		background: #0f0f0f;
		width: 100%;
		height: 100%;
		overflow: auto;
		/*padding-right: 17px; !* Increase/decrease this value for cross-browser compatibility *!*/
		/*box-sizing: content-box; !* So the width will be 100% + 17px *!*/
	}
</style>