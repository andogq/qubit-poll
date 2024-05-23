<script lang="ts">
	import Card from '$lib/Card.svelte';
	import type { Poll } from '$lib/server';
	import { total_votes } from '$lib/poll';
	import { PieChart, Plus } from 'lucide-svelte';

	export let poll_list: Poll[];
</script>

<div class="container">
	{#each poll_list as poll}
		<a href={`/${poll.id}`}>
			<Card title={poll.name} description={poll.description}>
				<div class="summary">
					<div class="icon">
						<PieChart size="1rem" />
					</div>

					<span>{Object.keys(poll.options).length} options, {total_votes(poll)} votes</span>
				</div>
			</Card>
		</a>
	{/each}

	<a href="/new" class="new">
		<Plus />

		<span>New Poll</span>
	</a>
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
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
	}
</style>
