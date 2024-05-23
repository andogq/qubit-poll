import api from '$lib/api';
import { readable, type Readable } from 'svelte/store';

export function create_result_store(poll_id: number): Readable<number[]> {
	return readable([] as number[], (set) => {
		return api.polls.poll_votes(poll_id).subscribe({
			on_data: (value) => {
				set(value);
			}
		});
	});
}
