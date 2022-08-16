<svelte:head>
	<title>Projets</title>
</svelte:head>

<script context="module">
	export const load = async ({ fetch }) => {
        const response = await fetch('/projects.json')

        if (response.ok) {
			const json = await response.json();
			const projects = json.projects;

			return {
				props: { projects }
			};
		}

        const { message } = await response.json();

        return {
            error: new Error(message)
        };
	};
</script>

<script>
    import { enhance } from '$lib/form';
    import { goto } from '$app/navigation';
    import { apiDirect } from '$lib/services/_server.js';
    import Search from "svelte-search";
    import Time from "svelte-time";

    export let projects;

    let canAdd = false;
    let search = "";

    async function handleSearch() {
        const res = await apiDirect('projects/search', {
            query: search,
        }, "POST")

        projects = res.body.projects;
    }

    async function handleClearSearch() {
        const res = await apiDirect('projects')

        projects = res.body.projects;
    }
</script>

<article class="max-w-6xl mx-auto py-12">
    <div class="flex justify-between">
        <h2 class="text-4xl mb-4">Projets</h2>

        <div>
            <button on:click={() => canAdd = !canAdd} type="button" class="border border-gray-300 hover:border-gray-400 hover:border-blue-700 hover:text-blue-700 rounded px-4 py-2">
                Ajouter
            </button>
        </div>
    </div>

    <!-- Add project form -->
    {#if canAdd}
         <section class="mt-10 sm:mt-0">
             <div class="md:grid md:grid-cols-3 md:gap-6">
                 <div class="mt-5 md:mt-0 md:col-span-2">
                     <form 
                        action="/projects.json" 
                        method="POST"
                        use:enhance={{
                            result: async (res, form) => {
                                if (res.ok) {
                                    goto('/projects')
                                }
                            }
                        }}
                    >
                         <div class="shadow overflow-hidden sm:rounded-md">
                             <div class="px-4 py-5 bg-white sm:p-6">
                                 <div class="grid grid-cols-6 gap-6">
                                     <div class="col-span-6">
                                         <label for="name" class="block text-sm font-medium text-gray-700">Nom du projet</label>
                                         <input type="text" name="name" id="name" autocomplete="given-name"
                                             class="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
                                     </div>
         
                                     <div class="col-span-6">
                                         <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
                                         <textarea id="description" name="description" rows="3" class="shadow-sm block w-full focus:ring-blue-500 focus:border-blue-500 sm:text-sm border border-gray-300 rounded-md"></textarea>
                                     </div>
                                 </div>
                             </div>
                             <div class="px-4 py-3 bg-gray-50 text-right sm:px-6">
                                 <button type="submit"
                                     class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                                     Ajouter
                                 </button>
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
                        <th class="py-3 border-b text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                            <div class="relative rounded-md shadow-sm">
                                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none" aria-hidden="true">
                                    <svg class="mr-3 h-4 w-4 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" ariaHidden="true">
                                        <path fillRule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clipRule="evenodd"></path>
                                    </svg>
                                </div>
    
                                <Search bind:value={search} hideLabel debounce={500} on:type={handleSearch} on:clear={handleClearSearch} type="text" name="search" id="search" class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-9 sm:text-sm border-gray-300 rounded-md py-3" placeholder="Rechercher un projet..." />
                            </div>
                        </th>
    
                        <th
                            class="hidden md:table-cell px-6 py-3 border-b text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                            Dernière màj
                        </th>
    
                        <th class="pr-6 py-3 border-b text-right text-xs font-medium text-gray-500 uppercase tracking-wider"></th>
                    </tr>
                </thead>
    
                <tbody class="bg-white divide-y divide-gray-100">
                    {#each projects as project}
                         <tr>
                             <td class="px-6 py-3 max-w-0 w-full whitespace-nowrap text-sm font-medium text-gray-900">
                                 <div class="flex items-center space-x-3 lg:pl-2">
                                     <div class="flex-shrink-0 w-2.5 h-2.5 rounded-full bg-pink-600" aria-hidden="true"></div>
                                     <a href="/projects/{project.uuid}/documents" class="truncate hover:text-gray-600">
                                         <span class="font-bold">{project.name}</span>
                                         <span class="text-gray-400">#{project.number}</span>
                                         <span class="block text-gray-500 font-normal">{project.description}</span>
                                     </a>
                                     {#if project.status}
                                        <span class="inline-flex items-center px-3 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                            En cours
                                        </span>
                                     {/if}
                                 </div>
                             </td>
                             
                             <td class="hidden md:table-cell px-6 py-3 whitespace-nowrap text-sm text-gray-500 text-right">
                                <Time timestamp={new Date(project.created_at * 1000)} />
                             </td>
                             
                             <td class="px-6 py-3 whitespace-nowrap text-right text-sm font-medium">
                                 <a href="/projects/{project.uuid}/documents" class="text-blue-600 hover:text-blue-900">Voir le projet</a>
                             </td>
                         </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>
</article>