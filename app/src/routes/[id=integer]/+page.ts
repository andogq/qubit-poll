import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const id = Number(params.id);

	if (isNaN(id)) {
		redirect(302, '/');
	}

	return { id };
};
