<script lang="ts">
	export let title: string;
	export let description: string;
</script>

<div class="container">
	<div class="header">
		<h2>{title}</h2>

		<p>{description}</p>
	</div>

	<slot />
</div>

<style>
	.container {
		border-radius: var(--radius-2);
		border: var(--border-size-1) solid var(--surface-4);

		padding: var(--size-5);

		background: var(--surface-1);
	}

	.header {
		margin-bottom: var(--size-5);

		& > h2 {
			font-size: var(--font-size-5);
			margin-bottom: var(--size-2);
		}
	}
</style>
