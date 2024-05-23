import api from '$lib/api';
import type { Stream } from '@qubit-rs/client';
import { readable, type Readable } from 'svelte/store';
import type { PollSummary } from './server';

export function stream_store<T>(stream: Stream<T>, initial?: T): Readable<T> {
	return readable(initial, (set) => {
		return stream.subscribe({ on_data: set });
	});
}

export function create_result_store(poll_id: number): Readable<number[]> {
	return stream_store(api.polls.poll_votes(poll_id), []);
}

export function create_overview_store(): Readable<PollSummary[]> {
	return stream_store(api.polls.overview(), []);
}
