import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	const res = await fetch('http://0.0.0.0:3000');
	const todos = (await res.json()) as Todo[];

	return {
		todos
	};
};
