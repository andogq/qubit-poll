<script lang="ts">
	import Card from '$lib/Card.svelte';
	import type { Poll } from '$lib/api';
	import { PieChart } from 'lucide-svelte';

	export let poll_list: Poll[];
</script>

{#each poll_list as poll}
	<Card>
		<h2>{poll.name}</h2>
		<p>{poll.description}</p>

		<div class="summary">
			<div class="icon">
				<PieChart size="1rem" />
			</div>

			<span>{poll.options.length} options, 125 votes</span>
		</div>
	</Card>
{/each}

<style>
	h2 {
		font-size: var(--font-size-5);
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
</style>
