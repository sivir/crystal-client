<script lang="ts">
	import {http_request} from "./lib";

	type Challenge = {
		name: string,
		points: number
	}

	let challenges = {};

	http_request("lol-challenges/v1/challenges/local-player", x => {
		challenges = x;
	})
</script>

<main>
	whee
	<div id="chal">
		{#each Object.values(challenges) as challenge}
			<div>{challenge.name}</div>
		{/each}
	</div>
</main>

<style>
	#chal {
		width: 100%;
		height: 100%;
		overflow-y: scroll;
		padding-right: 17px; /* Increase/decrease this value for cross-browser compatibility */
		box-sizing: content-box; /* So the width will be 100% + 17px */
	}
</style>