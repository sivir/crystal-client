<script lang="ts">
    import { supabase } from "./lib";
    let search: string = "";
    let temp: string = "";
</script>

<main>
	search: <input bind:value={search} />
    <button on:click={() => {
        supabase.functions.invoke("get-user", {
            body: { summoner_name: search },
        }).then(x => {
            console.log("get_user", x);
            temp = x.data;
	    });
    }} >button</button>
    {temp}
</main>