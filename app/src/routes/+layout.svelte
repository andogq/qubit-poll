<script lang="ts">
	import '$lib/global.css';

	import PollList from './PollList.svelte';
</script>

<main>
	<div>
		<h1><a href="/">Polls</a></h1>
		<p>Check out the latest polls and vote on your favourite options.</p>
	</div>

	<PollList />

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
