import api from '$lib/api';
import { readable, type Readable } from 'svelte/store';

export function create_result_store(poll_id: number): Readable<Record<string, number>> {
	return readable({}, (set) => {
		return api.polls.subscribe(poll_id).subscribe({
			on_data: (value) => {
				set(value);
			}
		});
	});
}
