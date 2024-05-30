<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{
		submit: { name: string; description: string; private_form: boolean; options: string[] };
	}>();

	let name: string;
	let description: string;
	let private_form: boolean = false;
	let options: string[] = [''];
	let form_el: HTMLFormElement;

	// Always make sure there's a new option at the end of the list
	$: if (options[options.length - 1] !== '') {
		options.push('');
		options = options;
	}

	$: valid = name && description && options.length > 1;

	function filter_options() {
		options = options.filter((value) => value.length > 0);
		options.push('');
		options = options;
	}

	function create_poll() {
		dispatch('submit', {
			name,
			description,
			private_form,
			options: options.slice(0, options.length - 1)
		});
	}

	export function clear() {
		name = '';
		description = '';
		options = [];
	}

	export function scroll_into_view() {
		form_el.scrollIntoView({ behavior: 'smooth' });
	}
</script>

<Card title="Create Poll" description="Fill out the form to create a new poll.">
	<form bind:this={form_el}>
		<label>
			<span>Poll Name</span>
			<input type="text" bind:value={name} placeholder="Enter poll name" />
		</label>

		<label>
			<span>Poll Description</span>
			<textarea bind:value={description} placeholder="Enter poll description"></textarea>
		</label>

		<label>
			<input type="checkbox" bind:checked={private_form} />
			<span>Private Form</span>
		</label>

		<div class="options">
			<span>Poll Options</span>

			{#each options as o}
				<input type="text" bind:value={o} placeholder="New Option" on:blur={filter_options} />
			{/each}
		</div>

		<button on:click={create_poll} type="submit" disabled={!valid}>Create</button>
	</form>
</Card>

<style>
	form {
		display: flex;
		flex-direction: column;
		gap: var(--size-3);

		& > label {
			width: 100%;

			&:not(:has(input[type='checkbox'])) {
				& > span {
					display: block;
					margin-bottom: var(--size-2);
				}

				& > input,
				& > textarea {
					width: 100%;
				}
			}

			&:has(input[type='checkbox']) {
				& > span {
					margin-left: var(--size-2);
				}
			}
		}

		& > button[type='submit'] {
			width: 100%;
		}
	}

	.options {
		display: flex;
		flex-direction: column;
		gap: var(--size-2);

		& > * {
			width: 100%;
		}
	}
</style>
