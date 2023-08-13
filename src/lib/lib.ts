type Challenge = {
    name: string,
    points: number;
    currentLevel: number;
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