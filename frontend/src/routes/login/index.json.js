import { api } from '$lib/services/_server.js';

export const post = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `users/login`, {
		email: data.get('email'),
		password: data.get('password'),
	});

	return respond(response);
}

function respond(response) {
	if (response.status !== 200) {
		return { response };
	}

	response.headers = {
		'set-cookie': `jwt=${response.body.token}; Path=/; HttpOnly`,
	}

	return response
}