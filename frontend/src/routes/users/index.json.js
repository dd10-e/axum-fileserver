
import { api } from '$lib/services/_server.js';

// GET /users.json
export const get = async (event) => {
	const response = await api(event, `users`);

	if (response.status === 404) {
		// user hasn't created a todo list.
		// start with an empty array
		return { body: [] };
	}

	return response;
};

// POST /users.json
export const post = async (event) => {
	const data = await event.request.formData();
	const response = await api(event, `users/register`, {
		// because index.svelte posts a FormData object,
		// request.body is _also_ a (readonly) FormData
		// object, which allows us to get form data
		// with the `body.get(key)` method
		first_name: data.get('first_name'),
		last_name: data.get('last_name'),
		email: data.get('email'),
		password: data.get('password'),
	});

	return response;
};