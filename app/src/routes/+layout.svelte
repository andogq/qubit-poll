<script lang="ts">
	import '$lib/global.css';

	import PollList from './PollList.svelte';
	import api from '$lib/api';

	let poll_list = api.polls.list();
</script>

<main>
	<div>
		<h1><a href="/">Polls</a></h1>
		<p>Check out the latest polls and vote on your favourite options.</p>
	</div>

	{#await poll_list then poll_list}
		<PollList {poll_list} />
	{/await}

	<div>
		<slot />
	</div>
</main>

<style>
	main {
		padding: var(--size-10) var(--size-11);

		display: flex;
		flex-direction: column;
		gap: var(--size-9);
	}

	h1 > a {
		color: var(--text-1);

		&:hover {
			text-decoration: none;
		}
	}
</style>
