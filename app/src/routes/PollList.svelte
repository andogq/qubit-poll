<script lang="ts">
	import Card from '$lib/Card.svelte';
	import api from '$lib/api';
	import { stream_store } from '$lib/store';
	import { PieChart, Plus } from 'lucide-svelte';

	let overview = stream_store(api.stream.overview(), []);
	let poll_totals = stream_store(api.stream.poll_total(), []);
</script>

<div class="container">
	{#each $overview as poll}
		<a href={`/${poll.id}`} data-sveltekit-noscroll>
			<Card title={poll.name} description={poll.description} --description-clamp={2}>
				<div class="summary">
					<div class="icon">
						<PieChart size="1rem" />
					</div>

					<span>{poll.options.length} options, {$poll_totals[poll.id]} votes</span>
				</div>
			</Card>
		</a>
	{/each}

	<a href="/new" class="new" data-sveltekit-noscroll>
		<Plus />

		<span>New Poll</span>
	</a>
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(var(--size-14), 1fr));
		gap: var(--size-4);
	}

	.summary {
		display: flex;
		flex-direction: row;
		align-items: center;

		margin-top: var(--size-5);

		color: var(--text-2);

		& > .icon {
			font-size: var(--font-size-1);
			margin-right: var(--size-2);
		}
	}

	a,
	a:visited,
	a:hover {
		color: var(--text-1);
		text-decoration: none;
	}

	.new {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		min-height: var(--size-12);

		border-radius: var(--radius-2);

		&:hover {
			background: var(--surface-3);
		}
	}
</style>
