<script lang="ts">
	import PollForm from '../PollForm.svelte';
	import api from '$lib/api';
	import { afterNavigate, goto } from '$app/navigation';

	let poll_form: PollForm;

	async function create_poll(
		e: CustomEvent<{ name: string; description: string; private_form: boolean; options: string[] }>
	) {
		let id = await api.create(
			e.detail.name,
			e.detail.description,
			e.detail.private_form,
			e.detail.options
		);
		goto(`/${id}`);
	}

	afterNavigate(() => {
		poll_form.scroll_into_view();
	});
</script>

<PollForm bind:this={poll_form} on:submit={create_poll} />
