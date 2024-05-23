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
	@import 'open-props/media';

	main {
		max-width: 1600px;
		margin: 0 auto;
		padding: var(--size-fluid-6) var(--size-fluid-5);

		@media (--md-n-below) {
			padding: var(--size-fluid-4) var(--size-fluid-3);
		}

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
