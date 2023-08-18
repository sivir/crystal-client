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
}

export let state: State = {
	champion_data: {},
	challenge_data: {},
	phase: ""
};