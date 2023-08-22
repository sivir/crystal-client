import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export type Challenge = {
    name: string;
	id: number;
	category: string;
	description: string;
    currentLevel: number;
	completedIds: number[];
	availableIds: number[];
	idListType: string;
}

type State = {
	champion_data: unknown;
	challenge_data: {
		[key: number]: Challenge
	};
	phase: string;
	lobby: string[];
	champ_select: string[];
}

export let state: Writable<State> = writable({
	champion_data: {},
	challenge_data: {},
	phase: "None",
	lobby: [],
	champ_select: []
});