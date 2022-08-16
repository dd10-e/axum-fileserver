<svelte:head>
	<title>Utilisateurs</title>
</svelte:head>

<script context="module">
	export const load = async ({ fetch }) => {
        const response = await fetch('/users.json')

		if (response.ok) {
			const res = await response.json();
			const users = res.users;

            return {
				props: { users }
			};
		}

		const { message } = await response.json();

		return {
			error: new Error(message)
		};
	};
</script>

<script>
    import { fade } from 'svelte/transition';
    import { invalidate } from '$app/navigation';

	import { enhance } from '$lib/form';
    import { apiDirect } from '$lib/services/_server.js';
    import Search from "svelte-search";

    export let users;

    let canAdd = false;
    let search = "";

    
    let is_sending;
    let add_user_error;
    let add_user_response;

    async function handleSearch() {
        const res = await apiDirect('users.json', {
            query: search,
        }, "POST")

        users = res.body.users;
    }

    async function handleClearSearch() {
        const res = await apiDirect('users.json')

        users = res.body.users;
    }
</script>

<article class="max-w-6xl mx-auto py-12">
    <div class="flex justify-between">
        <h2 class="text-4xl mb-4">Utilisateurs</h2>

        <div>
            <button on:click={() => canAdd = !canAdd} type="button" class="border border-gray-300 hover:border-gray-400 hover:border-blue-700 hover:text-blue-700 rounded px-4 py-2">
                Ajouter
            </button>
        </div>
    </div>

    <!-- Add user form -->
    {#if canAdd}
        <section class="mt-10 sm:mt-0">
            <div class="md:grid md:grid-cols-3 md:gap-6">
                <div class="mt-5 md:mt-0 md:col-span-2">
                    <form
                        action="/users.json"
                        method="POST"
                        use:enhance={{
                            pending: async (res) => {
                                is_sending = true;
                                add_user_error = null;
                            },
                            result: async (res, form) => {
                                let response = await res.json();
    
                                add_user_response = "Utilisateur ajouté.";
                                is_sending = false;
    
                                setTimeout(() => {
                                    add_user_response = null;
                                    invalidate('/users');
                                }, 1000);
                                
                            },
                            error: async(res) => {
                                let error_response = await res.json();
    
                                is_sending = false;
                                add_user_error = error_response.error;
                            }
                        }}
                    >
                        <div class="shadow overflow-hidden sm:rounded-md">
                            <div class="px-4 py-5 bg-white sm:p-6">
                                <div class="grid grid-cols-6 gap-6">
                                    <div class="col-span-6 sm:col-span-3">
                                        <label for="first_name" class="block text-sm font-medium text-gray-700">Prénom</label>
                                        <input type="text" name="first_name" id="first_name" autocomplete="given-name"
                                            class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                    </div>
        
                                    <div class="col-span-6 sm:col-span-3">
                                        <label for="last_name" class="block text-sm font-medium text-gray-700">Nom</label>
                                        <input type="text" name="last_name" id="last_name" autocomplete="family-name"
                                            class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                    </div>
        
                                    <div class="col-span-6 sm:col-span-4">
                                        <label for="email" class="block text-sm font-medium text-gray-700">
                                            Adresse email
                                        </label>
                                        <input type="text" name="email" id="email" autocomplete="email"
                                            class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                    </div>
        
                                    <div class="col-span-6 sm:col-span-3">
                                        <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                                        <input type="password" name="password" id="password" class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                    </div>
                                </div>
                            </div>
                            <div class="flex justify-end px-4 py-3 bg-gray-50 text-right sm:px-6">
                                <div class="flex items-center">
                                    {#if add_user_response}
                                        <p transition:fade class="mr-6 text-green-700">{add_user_response}</p>
                                    {/if}
                                    <button type="submit"
                                        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                                        Ajouter
                                    </button>
                                </div>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </section>
    {/if}
    
    <section class="hidden my-8 sm:block">
        <div class="align-middle inline-block min-w-full">
            <table class="min-w-full">
                <thead>
                    <tr>
                        <th class="py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                            <div class="relative rounded-md shadow-sm">
                                <Search bind:value={search} hideLabel debounce={500} on:type={handleSearch} on:clear={handleClearSearch} type="text" name="search" id="search" class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-9 sm:text-sm border-gray-300 rounded-md py-3" placeholder="Rechercher un projet..." />
                            </div>
                        </th>
    
                        <th
                            class="hidden md:table-cell px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                            Dernière màj
                        </th>
    
                        <th class="text-right text-xs font-medium text-gray-500 uppercase tracking-wider"></th>
                    </tr>
                </thead>

                <tbody class="bg-white border-t">
                    {#each users as user}
                         <tr>
                             <td class="px-6 py-3 max-w-0 w-full whitespace-nowrap text-sm font-medium text-gray-900">
                                 <div class="flex items-center space-x-3 lg:pl-2">
                                     <div class="flex-shrink-0 w-2.5 h-2.5 rounded-full bg-pink-600" aria-hidden="true"></div>
                                     <a href="/users/{user.uuid}" class="truncate hover:text-gray-600">
                                         <span class="font-bold">{user.first_name} {user.last_name}</span>
                                         <span class="text-gray-400">#{user.id}</span>
                                     </a>
                                 </div>
                             </td>
                             
                             <td class="hidden md:table-cell px-6 py-3 whitespace-nowrap text-sm text-gray-500 text-right">
                             </td>
                             
                             <td class="px-6 py-3 whitespace-nowrap text-right text-sm font-medium">
                                 <a href="/users/{user.uuid}" class="text-blue-600 hover:text-blue-900">Voir</a>
                             </td>
                         </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>
</article>