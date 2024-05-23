import type { Stream } from '@qubit-rs/client';
import { readable, type Readable } from 'svelte/store';

export function stream_store<T>(stream: Stream<T>, initial: T): Readable<T> {
	return readable(initial, (set) => {
		return stream.subscribe({ on_data: set });
	});
}
