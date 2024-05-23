<script lang="ts">
	import Card from '$lib/Card.svelte';
	import api from '$lib/api';
	import type { PollOverview } from '$lib/server.js';
	import { stream_store } from '$lib/store.js';

	export let data;

	let poll: Promise<PollOverview | null>;
	let vote: number | undefined;

	$: results = stream_store(api.stream.poll(data.id), []);

	$: {
		// Get the poll information
		poll = api.get_summary(data.id);

		// Clear any selection
		reset_vote();
	}

	function reset_vote() {
		vote = undefined;
	}

	async function make_vote() {
		if (vote === undefined) {
			return;
		}

		api.vote(data.id, vote);
		reset_vote();
	}
</script>

{#await poll then poll}
	{#if poll}
		{@const poll_votes = $results.reduce((total, vote) => total + vote, 0)}

		<Card title={poll.name} description={poll.description}>
			<form on:submit|preventDefault={make_vote}>
				{#each poll.options as option, i}
					{@const p = ($results[i] / (poll_votes || 1)) * 100}

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
