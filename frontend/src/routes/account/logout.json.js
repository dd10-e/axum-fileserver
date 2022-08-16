export function get(event) {
	return {
		headers: {
			'set-cookie': 'jwt=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT'
		},
        status: 200,
		body: {
			ok: true
		}
	};
}