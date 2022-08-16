<svelte:head>
	<title>Paramètres</title>
</svelte:head>

<script context="module">
	import { enhance } from '$lib/form';
    import { apiDirect, baseApi } from '$lib/services/_server.js';
    import { page } from '$app/stores';

    export const load = async ({ params, fetch }) => {
        const res = await apiDirect(`projects/${params.uid}`)
        const project = res.body;

        return {
            props: { project }
        };
	};
</script>

<script>
    import { goto } from '$app/navigation';
    import ProjectTopMenu from "$lib/filemanager/ProjectTopMenu.svelte";

    export let project;

    function deleteProject() {
        let isConfirmed = confirm(`Êtes-vous sûr de vouloir supprimmer le projet « ${project.name} »`);

        if (isConfirmed) {
            fetch(`${baseApi}/projects/${project.uuid}`, {}, 'DELETE')

            goto('/projects')
        }
    }
</script>

<article class="max-w-6xl mx-auto py-12">
    <ProjectTopMenu project={project} />

    <section class="bg-white shadow rounded container mx-auto">
        <div class="p-8">
            <div class="max-w-lg space-y-4">
                <!-- General informations -->
                <div>
                    <label for="name" class="block text-sm font-medium text-gray-700">
                        Nom du projet
                    </label>
    
                    <form 
                        action={`/projects/${project.uuid}.json?_method=PUT`}
                        method="post"
                        class="mt-1 flex space-x-2"
                        use:enhance={{
                            result: async (res, form) => {
                                if (res.ok) {
                                    console.log(res)
                                    goto('/projects')
                                }
                            }
                        }}
                    >
                        <input bind:value={project.name} type="text" name="name" id="name" class="block w-full shadow-sm focus:ring-sky-500 focus:border-sky-500 sm:text-sm border-gray-300 rounded-md" />
                        <input bind:value={project.description} type="hidden" name="description" id="description" class="block w-full shadow-sm focus:ring-sky-500 focus:border-sky-500 sm:text-sm border-gray-300 rounded-md" />
                        <button type="submit" class="whitespace-nowrap inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                            Mettre à jour
                        </button>
                    </form>
                </div>

                <!-- Dangerous parameters -->
                <div class="pt-12">
                    <h2 class="text-lg leading-6 font-medium text-red-900">
                        Zone dangereuse
                    </h2>

                    <div class="p-6 mt-3 space-y-6 border border-red-300 rounded">
                        <div class="bg-white shadow sm:rounded-lg">
                            <div class="px-4 py-5 sm:p-6">
                                <h3 class="text-lg leading-6 font-medium text-gray-900">
                                    Supprimer le projet
                                </h3>

                                <div class="mt-2 max-w-xl text-sm text-gray-500">
                                    <p>
                                        Une fois que vous supprimez ce projet, vous perdrez toutes les données qui y sont associées. 
                                    </p>
                                </div>
                                
                                <div class="mt-5">
                                    <button on:click={deleteProject}  type="button" class="inline-flex items-center justify-center px-4 py-2 border border-transparent font-medium rounded-md text-red-700 bg-red-100 hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:text-sm">
                                        Supprimer le projet
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</article>