<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";

	type Champion = {
		name: string,
		mastery: number,
		challenges: { [key: number]: boolean }
	}

	invoke("update_challenge_data").then(() => {
		invoke("get_challenge_data").then(x => console.log(x));
		}
	)

</script>

<main>
	champtable
</main>