
import { api } from '$lib/services/_server.js';

export const get = async (event) => {
	const response = await api(event, `projects`);

	if (response.status === 404) {
		// user hasn't created a todo list.
		// start with an empty array
		return { body: [] };
	}


	return response;
}

// POST /projects.json
export const post = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `projects`, {
		// because index.svelte posts a FormData object,
		// request.body is _also_ a (readonly) FormData
		// object, which allows us to get form data
		// with the `body.get(key)` method
		name: data.get('name'),
		description: data.get('description'),
	});

	return response;
};