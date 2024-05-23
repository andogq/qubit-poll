<script lang="ts">
	import PollForm from '../PollForm.svelte';
	import api from '$lib/api';

	let poll_form: PollForm;

	async function create_poll(
		e: CustomEvent<{ name: string; description: string; options: string[] }>
	) {
		await api.polls.create(e.detail.name, e.detail.description, e.detail.options);
		poll_form.clear();
	}
</script>

<PollForm bind:this={poll_form} on:submit={create_poll} />
