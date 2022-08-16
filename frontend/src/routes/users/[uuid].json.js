
import { api } from '$lib/services/_server.js';

export const post = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `users/${data.get('uuid')}`, {
		uuid: data.get('uuid'),
		first_name: data.get('first_name'),
		last_name: data.get('last_name'),
		email: data.get('email'),
		password: data.get('password'),
	});

	return response;
}

export const del = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `users/${data.get('uuid')}`);

	return response;
}