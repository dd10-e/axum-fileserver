
import { api } from '$lib/services/_server.js';

export const put = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `projects/${event.locals.uid}`, {
		name: data.get('name'),
		description: data.get('description'),
	});

	return response;
};
