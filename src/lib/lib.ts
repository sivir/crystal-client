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
	thresholds: {[key: string]: {value: number}};
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
	[key: number]: Challenge;
}

type Title = {
	name: string;
	itemId: number;
}

type State = {
	riot_id: string;
	lockfile_exists: boolean;
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
	titles: {
		[key: string]: Title
	}
	challenge_info: {
		[key: string]: {
			name: string;
			description: string;
			capstone: boolean;
			parent: string;
			thresholds: {
				[key: string]: {
					value: number;
				}
			}
		}
	}
}

let state: Writable<State> = writable({
	riot_id: "#",
	lockfile_exists: false,
	challenge_data: {},
	phase: "None",
	lobby: [],
	champ_select: [],
	champion_names: {},
	titles: {},
	champion_dragon: { data: {} },
	champ_shards: [],
	table_challenges: [],
	challenge_info: {}
});

const supabase = createClient('https://jvnhtmgsncslprdrnkth.supabase.co', 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imp2bmh0bWdzbmNzbHByZHJua3RoIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTQ2Mjc4ODMsImV4cCI6MjAxMDIwMzg4M30.OOjwsPjGHEc-x8MlhrOX64tJTNENqKqEq2635HKErrk');

export { state, supabase };
export type { Challenge, CommunityDragonChampion, ChallengeData };