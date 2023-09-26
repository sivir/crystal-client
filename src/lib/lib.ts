import { createClient } from "@supabase/supabase-js";
import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

type Challenge = {
    name: string;
	id: number;
	category: string;
	description: string;
    currentLevel: number;
	completedIds: number[];
	availableIds: number[];
	idListType: string;
	gameModes: string[];
}

type CommunityDragonChampion = {
	data: {
		[key: string]: {
			name: string,
			key: string
		}
	}
}

type ChampionShard = {
	count: number;
	storeItemId: number;
}

type ChallengeData = {
	[key: number]: Challenge
};

type State = {
	puuid: string;
	champion_data: unknown;
	challenge_data: ChallengeData;
	phase: string;
	lobby: string[];
	champ_select: number[];
	champion_dragon: CommunityDragonChampion;
	champion_names: {
		[key: number]: string
	}
	champ_shards: ChampionShard[];
	table_challenges: Challenge[];
}

let state: Writable<State> = writable({
	puuid: "",
	champion_data: {},
	challenge_data: {},
	phase: "None",
	lobby: [],
	champ_select: [],
	champion_names: {},
	champion_dragon: { data: {} },
	champ_shards: [],
	table_challenges: []
});

const supabase = createClient('https://jvnhtmgsncslprdrnkth.supabase.co', 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imp2bmh0bWdzbmNzbHByZHJua3RoIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTQ2Mjc4ODMsImV4cCI6MjAxMDIwMzg4M30.OOjwsPjGHEc-x8MlhrOX64tJTNENqKqEq2635HKErrk');

export { state, supabase };
export type { Challenge, CommunityDragonChampion, ChallengeData };