<script lang="ts">
	import Card from '$lib/Card.svelte';
	import api from '$lib/api';
	import type { Poll } from '$lib/server.js';

	export let data;

	let poll: Promise<Poll | null>;
	let vote: number | undefined;

	$: {
		poll = api.polls.get(data.id);
		reset_vote();
	}

	function reset_vote() {
		vote = undefined;
	}
</script>

{#await poll then poll}
	{#if poll}
		<Card title={poll.name} description={poll.description}>
			<form on:submit|preventDefault>
				{#each poll.options as option, i}
					<label class="btn">
						<input type="radio" name={`${poll.id}-value`} value={i} bind:group={vote} />
						<span>
							{option}
						</span>
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
	}
</style>
