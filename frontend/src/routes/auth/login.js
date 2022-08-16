import {serialize} from 'cookie'
import {dev} from '$app/env'
import { api } from '$lib/services/_server.js';

export const post = async (event) => {
	const data = await event.request.formData();

	const response = await api(event, `users/login`, {
		email: data.get('email'),
		password: data.get('password'),
	});
    

    const cookie = createCookie({name:'connect.a.id', value: response.body.token})
    
    return {
        status: 200,
        headers: {
            'set-cookie': cookie
        },
    }
}

function createCookie({name,value,domain}){
    let expires = new Date()

    expires.setMonth(expires.getMonth() + 1) // setting cookie to expire in 1 months

    let cookie_options = {httpOnly: true, path:'/', sameSite:true, expires}

    if( !dev ) {
        cookie_options.secure = true
        cookie_options.domain = domain
    }

    const cookie = serialize(name,value,cookie_options)
    
    return cookie
  }