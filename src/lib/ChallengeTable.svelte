<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
	import {state} from "./lib";

	$: state && console.log(state);

	invoke("update_all_data").then(() => {
		invoke("get_challenge_data").then(x => {
			state.challenge_data = x as any;
			console.log("chall", state.challenge_data);
		})
	});
</script>

<main>
	whee
	<div>
		<table >
			{#each Object.entries(state.challenge_data) as [_, challenge]}
				<tr>
					<td>{challenge.name}</td>
					<td>{challenge.currentLevel}</td>
				</tr>
			{/each}
		</table>
	</div>
</main>

<style>
</style>