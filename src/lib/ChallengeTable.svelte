<script lang="ts">
	import {http_request} from "./lib";

	type Challenge = {
		name: string,
		points: number
	}

	let challenges = {};

	http_request("lol-challenges/v1/challenges/local-player", x => {
		challenges = JSON.parse(x);
	})
</script>

<main>
	whee
	{#each [...Object.entries(challenges)] as [key, val]}
		<div>{JSON.stringify(val.name)}</div>
	{/each}
</main>