<script lang="ts">
	import { ws } from '@qubit-rs/client';
	import type { Server } from '$lib/api';
	import PollList from './PollList.svelte';
	import PollForm from './PollForm.svelte';

	const api = ws<Server>('ws://localhost:3030/api');

	let poll_list = api.polls.list();
</script>

<main>
	<div>
		<h1>Polls</h1>
		<p>Check out the latest polls and vote on your favourite options.</p>
	</div>

	<div class="poll_list">
		{#await poll_list then poll_list}
			<PollList {poll_list} />
		{/await}
	</div>

	<div>
		<slot />
	</div>

	<PollForm />
</main>

<style>
	:global(html, body) {
		font-family: 'Poppins', sans-serif;
	}

	main {
		padding: var(--size-10) var(--size-11);

		display: flex;
		flex-direction: column;
		gap: var(--size-5);
	}

	.poll_list {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--size-4);
	}
</style>
