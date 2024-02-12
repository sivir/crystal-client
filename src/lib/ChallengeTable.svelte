<script lang="ts">
	import {state} from "./lib";
</script>

<main>
	<div>
		<table>
			{#each Object.entries($state.challenge_data) as [_, challenge]}
				<tr>
					<td>{challenge.name}</td>
					<td>{challenge.currentLevel}</td>
				</tr>
			{/each}
		</table>
	</div>
</main>