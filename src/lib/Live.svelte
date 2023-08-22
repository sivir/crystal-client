<script lang="ts">
	import {state} from "./lib";

	$: state && console.log(state);
</script>

<main>
	{JSON.stringify($state.phase)}
	{JSON.stringify($state.lobby)}
	{JSON.stringify($state.champ_select)}
</main>