<script lang="ts">
	import Card from '$lib/Card.svelte';
	import api from '$lib/api';
	import { total_votes } from '$lib/poll.js';
	import type { Poll } from '$lib/server.js';
	import { onDestroy } from 'svelte';

	export let data;

	let poll: Promise<Poll | null>;
	let vote: number | undefined;
	let results: Record<string, number> = {};

	let cancel_subscription: () => void | undefined;

	$: {
		// Get the poll information
		poll = api.polls.get(data.id);

		// Capture initial results when poll loads
		poll.then((poll) => {
			results = poll!.options;
		});

		// Stop any on-going subscription
		if (cancel_subscription) {
			cancel_subscription();
		}

		// Subscribe to vote changes
		cancel_subscription = api.polls.subscribe(data.id).subscribe({
			on_data: (votes) => {
				// Save incomming vote information
				results = votes;
			}
		});

		// Clear any selection
		reset_vote();
	}

	onDestroy(() => {
		if (cancel_subscription) {
			cancel_subscription();
		}
	});

	function reset_vote() {
		vote = undefined;
	}

	async function make_vote() {
		await api.polls.vote(data.id, Object.keys((await poll)!.options)[vote!]);
	}
</script>

{#await poll then poll}
	{#if poll}
		{@const poll_votes = total_votes(results)}

		<Card title={poll.name} description={poll.description}>
			<form on:submit|preventDefault={make_vote}>
				{#each Object.entries(results) as [option, votes], i}
					{@const p = (votes / (poll_votes || 1)) * 100}

					<label class="btn" style:--fill={`${p}%`}>
						<input type="radio" name={`${poll.id}-value`} value={i} bind:group={vote} />
						<span>
							{option}
						</span>

						<span>{Math.round(p)}%</span>
					</label>
				{/each}

				<button type="submit" disabled={vote === undefined}>Vote</button>
			</form>
		</Card>
	{:else}
		<Card
			title="Poll not found"
			description="A poll with that ID could not be found. Please try again."
		/>
	{/if}
{/await}

<style>
	@import 'open-props/media';

	form {
		display: flex;
		flex-direction: column;
		gap: var(--size-3);
	}

	label:has(input[type='radio']) {
		cursor: pointer;

		justify-content: flex-start;

		&:not(:has(input:checked)) > span {
			font-weight: normal;
		}

		--poll-color: var(--blue-1);

		@media (--OSdark) {
			--poll-color: var(--blue-9);
		}

		background: linear-gradient(
			to right,
			var(--poll-color),
			var(--poll-color) var(--fill),
			transparent var(--fill),
			transparent 100%
		);
	}
</style>
