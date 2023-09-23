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

type State = {
	champion_data: unknown;
	challenge_data: {
		[key: number]: Challenge
	};
	phase: string;
	lobby: string[];
	champ_select: number[];
	champion_dragon: CommunityDragonChampion;
	champ_shards: ChampionShard[];
	table_challenges: Challenge[];
}

let state: Writable<State> = writable({
	champion_data: {},
	challenge_data: {},
	phase: "None",
	lobby: [],
	champ_select: [],
	champion_dragon: { data: {} },
	champ_shards: [],
	table_challenges: []
});

export { state };
export type { Challenge, CommunityDragonChampion };