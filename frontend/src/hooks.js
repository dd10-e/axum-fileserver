import cookie from 'cookie';
import { v4 as uuid } from '@lukeed/uuid';
import jsonwebtoken from 'jsonwebtoken'

export const handle = async ({ event, resolve }) => {
	const cookies = cookie.parse(event.request.headers.get('cookie') || '');
	event.locals.userid = cookies.userid || uuid();

	// Verify authentication
	if (cookies.jwt) {
		const jwt = jsonwebtoken.verify(cookies.jwt, 'secret');
		event.locals.user = jwt ? jwt : null;
	}

	const response = await resolve(event);

	if (!cookies.userid) {
		// if this is the first time the user has visited this app,
		// set a cookie so that we recognise them when they return
		response.headers.set(
			'set-cookie',
			cookie.serialize('userid', event.locals.userid, {
				path: '/',
				httpOnly: true
			})
		);
	}

	return response;
};

export function getSession({ locals }) {
	return {
		user: locals.user && {
			...locals.user
		}
	};
}