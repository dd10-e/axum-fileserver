<script>
    import { invalidate } from '$app/navigation';
	import Time from "svelte-time";
	import { dayjs } from "svelte-time";
    // import 'dayjs/esm/locale/fr.js'
    import { page } from '$app/stores';

	import { apiDirect } from '$lib/services/_server.js';

	// dayjs.locale("fr");

	export let project;
	export let isNotShared;
    export let document;
	export let document_index;
	export let qwickActionsMenuItemId = null;
	export let hasOneSelectedItem;
	export let bindGroup = [];
	export let value = document_index;

    $: checked = bindGroup.includes(value);

    let path_url = $page.params.index;
    if ($page.params.index === "") {
        path_url = document.name;
    } else {
        path_url = `${$page.params.index}/${document.name}`;
    }

    let document_url = `/projects/${project.uuid}/documents/${path_url}`;
    if (! isNotShared) {
        document_url = document_url + '?share=true'
    }

	function onChange() {
        if (hasOneSelectedItem === false) {
            hasOneSelectedItem = true;
        }

		if (checked === false) {
			bindGroup = [...bindGroup, value]
		} else {
			bindGroup = bindGroup.filter((item) => item !== value);
		}

        if (bindGroup.length === 0) {
            hasOneSelectedItem = false;
        }
	}

    function showQwickActionsMenuItem(id) {
		if (id === qwickActionsMenuItemId) {
			return qwickActionsMenuItemId = null	
		}

		return qwickActionsMenuItemId = id
	}

    async function delete_document(document_name) {
		await apiDirect(`fileserver/documents/${project.uuid}/delete/${path_url}`, {}, 'POST')

        invalidate(`/projects/${project.uuid}/documents/${$page.params.index}`)
	}
</script>

<tr class="outline-none hover:bg-gray-100" tabindex={document_index}>
    <td class="checkbox-column py-2 px-3 w-12">
        <input
            on:change={onChange}
            checked={checked}
            value={document_index}
            type="checkbox"
        />
    </td>
    
    <td class="py-3">
        <a href={document_url} class="flex items-center w-fit-content group">
            <div class="w-8 h-8 mr-1 cursor-pointer text-blue-500">
                {#if document.path_type === "Dir"}
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mr-2" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
                    </svg>
                {:else}
                    <!-- <img src="NlLWxlZS1waG90by5qcGc=/small" loading="lazy" class="asset-thumbnail rounded max-h-full max-w-full mx-auto rounded w-8 h-8 fit-cover"/> -->
                {/if}
            </div>

            <p class="cursor-pointer select-none text-blue-500 group-hover:underline text-md">
                {document.name}
            </p>
        </a>
    </td>
    
    <td class="py-3">
        <div>
            <p handle="size" values="[object Object]" class="text-gray-600 text-md">
                {document.size}
            </p>
        </div>
    </td>
    
    <td class="py-3">
        <div>
            <div
                handle="last_modified"
                class="text-gray-600 text-md"
            >
                <Time timestamp={new Date(document.creation_date * 1000)} relative live />
            </div>
        </div>
    </td>

    {#if isNotShared}
        <!-- Qwick Actions -->
        <td class="actions-column relative px-4">
            <div>
                <div>
                    <button 
                        on:click={() => showQwickActionsMenuItem('qwick-actions-' + document_index)}
                        type="button"
                        class="flex justify-center cursor-pointer hover:bg-gray-100"
                    >
                        <svg width="12" viewBox="0 0 24 24" class="rotating-dots fill-current">
                            <circle cx="3" cy="12" r="3"></circle>
                            <circle cx="12" cy="12" r="3"></circle>
                            <circle cx="21" cy="12" r="3"></circle>
                        </svg>
                    </button>
                </div>

                {#if qwickActionsMenuItemId === 'qwick-actions-' + document_index}
                    <div
                        class="absolute absolute right-0 top-full bg-white z-10"
                        data-popper-placement="top-end"
                        aria-label="Ouvrir"
                    >
                        <div class="shadow-lg p-2 space-y-2 popover-content bg-white shadow-popover rounded-md">
                            {#if document.path_type === "File"}
                                <a 
                                    download 
                                    href={`http://localhost:4000/fileserver/documents/${project.uuid}/download-file/${document.name}`} 
                                    class="block w-full text-left hover:bg-gray-200 px-2 py-1">
                                    Télécharger
                                </a>
                            {/if}

                            <div class="divider h-px bg-gray-300 -mx-2" />
                            
                            <div>
                                <!-- <button class="block w-full text-left hover:bg-gray-200 px-2 py-1">
                                    Déplacer
                                </button> -->
                                
                                <button class="block w-full text-left hover:bg-gray-200 px-2 py-1">
                                    Renommer
                                </button>
                                
                                <button on:click={() => delete_document(document.name)} class="block w-full text-left hover:bg-red-500 hover:text-white px-2 py-1">
                                    Supprimer
                                </button>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>
        </td>
    {/if}
</tr>