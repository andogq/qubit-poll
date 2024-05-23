<script lang="ts">
	import { ws } from '@qubit-rs/client';
	import type { Server } from '$lib/api';

	const api = ws<Server>('ws://localhost:3030/api');

	let poll_name: string;
	let poll_description: string;
	let poll_options: string[] = [];
	async function create_poll() {
		await api.polls.create(poll_name, poll_description, poll_options);
		poll_name = '';
		poll_options = [];
	}
</script>
