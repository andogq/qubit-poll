<script lang="ts">
	import { ws } from '@qubit-rs/client';
	import type { Server } from '$lib/api';

	const api = ws<Server>('ws://localhost:3030/api');

	let poll_list = api.polls.list();

	let poll_name: string;
	let poll_options: string[] = [];
	async function create_poll() {
		await api.polls.create(poll_name, poll_options);
		poll_name = '';
		poll_options = [];

		poll_list = api.polls.list();
	}

	let poll_option: string;
	function append_option() {
		poll_options.push(poll_option);
		poll_options = poll_options;
		poll_option = '';
	}
	function remove_option(i: number) {
		poll_options.splice(i, 1);
		poll_options = poll_options;
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

<br />

<ul>
	{#each poll_options as option, i}
		<button on:click={() => remove_option(i)}>x</button>
		<ol>{option}</ol>
	{/each}
</ul>

<label>
	<span>Option</span>
	<input type="text" bind:value={poll_option} />
</label>

<button on:click={append_option}>Add</button>

<br />

<button on:click={create_poll}>Create</button>
