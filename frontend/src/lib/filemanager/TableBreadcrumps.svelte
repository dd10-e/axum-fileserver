<script>
	import { page } from '$app/stores';

	export let breadcrumps = false;
</script>

<div class="bg-gray-50 px-6 py-2 flex flex-wrap border-t border-b space-x-3">
    <a href={`/projects/${$page.params.uid}/documents/`} class="group flex items-center">
        <svg class="flex-shrink-0 h-5 w-5 mr-1" x-description="Heroicon name: solid/home" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
            <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"></path>
        </svg>
        <span class="text-gray-600">Documents</span>
     </a>

    {#each breadcrumps as breadcrump, id}
        {#if id != 0}
            <a href={`/projects/${$page.params.uid}/documents/${breadcrump.path}`} class="group flex items-center">
                <svg class="flex-shrink-0 h-5 w-5 mr-1 text-gray-400" x-description="Heroicon name: solid/chevron-right" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
                </svg>
                <span class="text-gray-600">{breadcrump.name}</span>
            </a>
        {/if}
    {/each}
</div>