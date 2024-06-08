import { readable, type Readable } from 'svelte/store';

export function stream_store<T>(
	create_stream: (on_data: (data: T) => void) => () => void,
	initial: T
): Readable<T> {
	return readable(initial, (set) => {
		return create_stream(set);
	});
}
