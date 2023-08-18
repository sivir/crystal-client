<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {listen} from "@tauri-apps/api/event"
    import {appWindow} from '@tauri-apps/api/window'

    import ChallengeTable from "./lib/ChallengeTable.svelte";
    import ChampionTable from "./lib/ChampionTable.svelte";
    import Settings from "./lib/Settings.svelte";
    import Live from "./lib/Live.svelte";

    enum Page {
        champions, // champion table with mastery
        challenges, // just a list of challenges
        globe, // globetrotters + harmony list
        live, // live lobby then champ select data
        settings, // settings
    }

    let lockfile_exists = false;
    let gameflow = "None";
    let page = Page.champions;

    console.log("stuff");

    $: if (lockfile_exists) {
        invoke("process_lockfile");
        invoke("start_lcu_websocket", {endpoints: ["OnJsonApiEvent_lol-gameflow_v1_gameflow-phase"]});
        invoke("http_retry", {endpoint: "help"}).then(c => console.log("help", c));
    }

    listen("lockfile", x => {
        const payload = x.payload;
        console.log(payload, lockfile_exists);
        if (payload === "create" && lockfile_exists === false) {
            lockfile_exists = true;
            console.log("lockfile exists");
        } else if (payload === "remove" && lockfile_exists === true) {
            lockfile_exists = false;
            console.log("lockfile doesnt exist");
        }
    });
    listen("gameflow", x => {
        console.log(x);
        gameflow = x.payload.toString();
    });
    invoke("process_lockfile");
    invoke("async_watch");
</script>

<main>
    <div data-tauri-drag-region class="titlebar">
        <div class="titlebar-button">crystal</div>
        <div>
            <button class="titlebar-button" on:click={appWindow.minimize}>
                <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize"/>
            </button>
            <button class="titlebar-button" on:click={appWindow.toggleMaximize}>
                <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize"/>
            </button>
            <button class="titlebar-button" on:click={appWindow.hide}>
                <img src="https://api.iconify.design/mdi:close.svg" alt="close"/>
            </button>
        </div>
    </div>

    <div id="sideways">
        <div id="sidebar">
            <div id="sidebuttons">
                <button on:click={() => page = Page.challenges}>challenges</button>
                <button on:click={() => page = Page.champions}>champions</button>
                <button on:click={() => page = Page.settings}>settings</button>
            </div>
        </div>

        <div id="main">
            <p>lockfile {lockfile_exists ? "exists" : "doesn't exist"}. {gameflow}</p>
            <div class="test" hidden={page !== Page.champions}>
                <ChampionTable {lockfile_exists}/>
            </div>
            <div class="test" hidden={page !== Page.challenges}>
                <ChallengeTable/>
            </div>
            <div class="test" hidden={page !== Page.settings}>
                <Settings/>
            </div>
            <div class="test" hidden={page !== Page.live}>
                <Live/>
            </div>
        </div>
    </div>
</main>

<style>
    .test {
        height: 100%;
        overflow: auto;
    }

    #main {
        height: 100%;
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    main {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
    }

    #sidebar {
        height: 100%;
        background: #212121;
        width: 150px;
    }

    #sidebuttons {
        display: flex;
        flex-direction: column;
        gap: 20px;
        padding-top: 20px;
    }

    #sideways {
        flex: 1;
        overflow: auto;
        max-height: 100%;
        display: flex;
    }

    .titlebar {
        padding-left: 5px;
        padding-right: 5px;
        height: 30px;
        background: #1f1f1f;
        user-select: none;
        display: flex;
        justify-content: space-between;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        height: 30px;
    }

    .titlebar-button:hover {
        background: #5bbec3;
    }
</style>
