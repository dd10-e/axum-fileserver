<svelte:head>
	<title>Profil {user.first_name} {user.last_name}</title>
</svelte:head>

<script context="module">
    import { baseApi } from '$lib/services/_server.js';

	export const load = async ({ params, session }) => {
        const res = await fetch(`${baseApi}/users/${params.uuid}`)

		if (res.ok) {
			const user = await res.json();

            if (user.uuid === session.user.uuid) {
                // return {
                //     status: 301,
                //     headers: {
                //         location: `/account`
                //     }
                // };
            }

            return {
				props: { user }
			};
		}

		const { message } = await res.json();

		return {
			error: new Error(message)
		};
	};
</script>

<script>
    import { session } from '$app/stores';
    import { goto } from '$app/navigation';
    import { fade } from 'svelte/transition';

	import { enhance } from '$lib/form';

    export let user;

    let is_sending;
    let update_error;
    let update_response;

    let delete_is_sending;
    let delete_error;
    let delete_response;
</script>


<article class="max-w-6xl mx-auto py-12">
    <div class="flex justify-between">
        <h2 class="text-4xl mb-4">Profil de <span class="underline">{user.first_name} {user.last_name}</span></h2>
    </div>

    <section class="mt-10 sm:mt-0">
        <div class="md:grid md:grid-cols-3 md:gap-6">
            <div class="mt-5 md:mt-0 md:col-span-2">
                <form 
                    action={`/users/${user.uuid}.json`} 
                    method="POST"
                    use:enhance={{
						pending: async (res) => {
							is_sending = true;
							update_error = null;
						},
						result: async (res, form) => {
                            let response = await res.json();

                            user.email = response.email;
                            user.first_name = response.first_name;
                            user.last_name = response.last_name;
                            user.password = response.password;

                            update_response = "Utilisateur mis à jour.";
							is_sending = false;

                            setTimeout(() => {
                                update_response = null;
                            }, 3000);
						},
						error: async(res) => {
							let error_response = await res.json();

							is_sending = false;
							update_error = error_response.error;
						}
					}}
                >
                    <input type="hidden" name="uuid" value={user.uuid}>

                    <div class="shadow overflow-hidden sm:rounded-md">
                        <div class="px-4 py-5 bg-white sm:p-6">
                            <div class="grid grid-cols-6 gap-6">
                                <div class="col-span-6 sm:col-span-3">
                                    <label for="first_name" class="block text-sm font-medium text-gray-700">Prénom</label>
                                    <input type="text" name="first_name" id="first_name" required value={user.first_name}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-3">
                                    <label for="last_name" class="block text-sm font-medium text-gray-700">Nom</label>
                                    <input type="text" name="last_name" id="last_name" required value={user.last_name}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-4">
                                    <label for="email" class="block text-sm font-medium text-gray-700">
                                        Adresse email
                                    </label>
                                    <input type="text" name="email" id="email" required value={user.email}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-3">
                                    <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                                    <input type="password" name="password" id="password" required value={user.password} class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
                            </div>
                        </div>
                        <div class="flex justify-end items-center px-4 py-3 bg-gray-50 text-right sm:px-6 space-x-4">
                            <div class="flex items-center">
                                {#if update_response}
                                     <p transition:fade class="mr-6 text-green-700">{update_response}</p>
                                {/if}
                                
                                <button 
                                    type="submit"
                                    class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                                    {is_sending ? `Mise à jour...` : `Mettre à jour`}
                                </button>
                            </div>
                        </div>
                    </div>
                </form>

                <form
                    action={`/users/${user.uuid}.json?_method=DELETE`} 
                    method="POST"
                    use:enhance={{
                        pending: async (res) => {
                            delete_is_sending = true;
                            delete_error = null;
                        },
                        result: async (res, form) => {
                            let response = await res.json();

                            update_response = "Utilisateur supprimé. Vous allez être redirigé";
                            delete_is_sending = false;

                            setTimeout(() => {
                                update_response = null;

                                goto('/users')
                            }, 3000);
                        },
                        error: async(res) => {
                            let error_response = await res.json();

                            delete_is_sending = false;
                            delete_error = error_response.error;
                        }
                    }}
                >
                    <input type="hidden" name="uuid" value={user.uuid}>

                    {#if delete_response}
                        <p transition:fade class="mr-6 text-green-700">{delete_response}</p>
                    {/if}
                    <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-red-700">
                        {delete_is_sending ? `Suprresion...` : `Supprimer`}
                    </button>
                </form>
            </div>
        </div>
    </section>
</article>