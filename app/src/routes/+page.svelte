<script>
	import api from '$lib/api';
	api.hello_world().then(console.log);
</script>

<span>Select a poll to view results and vote.</span>

<style>
	span {
		color: var(--text-2);
		font-style: italic;
		font-size: var(--font-size-1);
		text-align: center;
		width: 100%;
		display: block;
	}
</style>
