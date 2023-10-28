<!-- FILEPATH: /c:/Users/Jeffrey/IdeaProjects/crystal-client/src/lib/Profile.svelte -->

<script>
    export let id;
</script>

<div class="card">
    <h2>Profile {id}</h2>
    <p>This is the profile with ID {id}.</p>
</div>

<style>
    .card {
        border: 1px solid #ccc;
        border-radius: 4px;
        padding: 16px;
        margin: 16px;
    }
</style>
