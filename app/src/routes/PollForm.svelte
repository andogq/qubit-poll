<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let name: string;
	let description: string;
	let options: string[] = [];

	let poll_option: string;
	function append_option() {
		options.push(poll_option);
		options = options;
		poll_option = '';
	}
	function remove_option(i: number) {
		options.splice(i, 1);
		options = options;
	}

	function create_poll() {
		dispatch('submit', { name, description, options });
	}
</script>

<Card>
	<h2>Create Poll</h2>

	<p>Fill out the form to create a new poll.</p>

	<label>
		<span>Poll Name</span>
		<input type="text" bind:value={name} placeholder="Enter poll time" />
	</label>

	<label>
		<span>Poll Description</span>
		<textarea bind:value={description} placeholder="Enter poll description"></textarea>
	</label>

	<br />

	<ul>
		{#each options as option, i}
			<button on:click={() => remove_option(i)}>x</button>
			<ol>{option}</ol>
		{/each}
	</ul>

	<label>
		<span>Option</span>
		<input type="text" bind:value={poll_option} placeholder="Option 1" />
	</label>

	<button on:click={append_option}>Add</button>

	<br />

	<button on:click={create_poll}>Create</button>
</Card>
