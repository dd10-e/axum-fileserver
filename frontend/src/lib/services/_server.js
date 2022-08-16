export const baseApi = import.meta.env.VITE_SITE_BACKEND_URL;

export async function api(event, resource, data) {
	const res = await fetch(`${baseApi}/${resource}`, {
		method: event.request.method,
		headers: {
			'content-type': 'application/json'
		},
		body: data && JSON.stringify(data)
	});

	return {
		status: res.status,
		body: await res.json()
	};
}

// TODO: remove
export async function apiDirect(resource, data, method) {
	const res = await fetch(`${baseApi}/${resource}`, {
		headers: {
			'content-type': 'application/json'
		},
		body: data && JSON.stringify(data),
		method: method || "GET"
	});

	return {
		status: res.status,
		body: await res.json()
	};
}