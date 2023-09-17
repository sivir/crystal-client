<script lang="ts">
	import {state} from "./lib";
</script>

<main>
	<p>state: {JSON.stringify($state.phase)}</p>
	<p>current lobby: {JSON.stringify($state.lobby)}</p>
	<p>current champ select: {JSON.stringify($state.champ_select)}</p>
</main>