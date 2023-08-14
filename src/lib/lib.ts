export type Challenge = {
    name: string;
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
}

export let state: State = {
	champion_data: {},
	challenge_data: {},
};