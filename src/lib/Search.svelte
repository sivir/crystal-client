<script lang="ts">
    import { empty } from "svelte/internal";
import {supabase, state} from "./lib";
    let search: string = "";
    let temp: string = "";
    let data = {
        riot_data: {
            challenges: [],
        },
        lcu_data: {},
    };
</script>

<main>
	search: <input bind:value={search} />
    <button on:click={() => {
        supabase.functions.invoke("get-user", {
            body: { summoner_name: search },
        }).then(x => {
            temp = x.data;
            data = JSON.parse(x.data);
            console.log("get_user", data);
	    });
    }} >button</button>
    {#if temp !== ""}
        <table>
            {#each Object.entries($state.challenge_data) as [_, challenge]}
                <tr>
                    <td>{challenge.name}</td>
                    <!-- <td>{data.riot_data.challenges.filter(x => x.challengeId === challenge.id)[0].level}</td> -->
                </tr>
            {/each}
        </table>
        {temp}
    {/if}
</main>