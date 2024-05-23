import type { Poll } from '$lib/server';

export function total_votes(poll: Poll['options']): number {
	return Object.values(poll).reduce((total, votes) => total + votes, 0);
}
