<script lang="ts">
	import { ws } from '@qubit-rs/client';
	import type { Server } from '$lib/api';

	const api = ws<Server>('ws://localhost:3030/api');

	const message = api.hello_world();
</script>

<h1>Qubit Poll</h1>

<p>
	Message from server:

	{#await message}
		<em>loading...</em>
	{:then message}
		<b>{message}</b>
	{/await}
</p>
