<script lang="ts">
	import { ws } from '@qubit-rs/client';
	import type { Server } from '$lib/api';

	const api = ws<Server>('ws://localhost:3030/api');

	let poll_list = api.polls.list();

	let poll_name: string;
	async function create_poll() {
		await api.polls.create(poll_name);
		poll_name = '';

		poll_list = api.polls.list();
	}
</script>

<h1>Qubit Poll</h1>

<h2>Available Polls</h2>

{#await poll_list then poll_list}
	<ul>
		{#each poll_list as poll}
			<li>{poll}</li>
		{/each}
	</ul>
{/await}

<h2>Create Poll</h2>

<label>
	<span>Poll Name</span>
	<input type="text" bind:value={poll_name} placeholder="Poll Name" />
</label>

<button on:click={create_poll}>Create</button>
