import type { Poll } from '$lib/server';

export function total_votes(poll: Poll): number {
	return Object.values(poll.options).reduce((total, votes) => total + votes, 0);
}
