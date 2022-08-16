<svelte:head>
	<title>Compte</title>
</svelte:head>

<script context="module">
    import { baseApi } from '$lib/services/_server.js';

	export const load = async ({ session }) => {
        const res = await fetch(`${baseApi}/users/${session.user.uuid}`)

		if (res.ok) {
			const user = await res.json();

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
    import { fade } from 'svelte/transition';

	import { enhance } from '$lib/form';

    export let user;

    let is_sending;
    let update_error;
    let update_response;
</script>


<article class="max-w-6xl mx-auto py-12">
    <div class="flex justify-between">
        <h2 class="text-4xl mb-4">Mon compte</h2>
    </div>

    <section class="mt-10 sm:mt-0">
        <div class="md:grid md:grid-cols-3 md:gap-6">
            <div class="mt-5 md:mt-0 md:col-span-2">
                <form 
                    action={`/account.json`} 
                    method="POST"
                    use:enhance={{
						pending: async (res) => {
							is_sending = true;
							update_error = null;
						},
						result: async (res, form) => {
                            let response = await res.json();

                            $session.user.email = response.email;

                            update_response = "Utilisateur ajouté.";
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
                                    <input type="text" name="first_name" id="first_name" value={user.first_name}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-3">
                                    <label for="last_name" class="block text-sm font-medium text-gray-700">Nom</label>
                                    <input type="text" name="last_name" id="last_name" value={user.last_name}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-4">
                                    <label for="email" class="block text-sm font-medium text-gray-700">
                                        Adresse email
                                    </label>
                                    <input type="text" name="email" id="email" value={user.email}
                                        class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
    
                                <div class="col-span-6 sm:col-span-3">
                                    <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                                    <input type="password" name="password" id="password" value={user.password} class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                </div>
                            </div>
                        </div>
                        <div class="flex justify-end px-4 py-3 bg-gray-50 text-right sm:px-6">
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
            </div>
        </div>
    </section>
</article>