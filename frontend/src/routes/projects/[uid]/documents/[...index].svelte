<svelte:head>
	<title>Documents</title>
</svelte:head>

<script context="module">
	
	import { apiDirect } from '$lib/services/_server.js';
	
	export async function load({ params, fetch }) {
		const fileserverResponse = await apiDirect(`fileserver/documents/${params.uid}/serve/${params.index}`);
		const fileserver = await fileserverResponse.body;

		const projectResponse = await apiDirect(`projects/${params.uid}`);
		const project = await projectResponse.body;

		const documents = fileserver.documents;
		const breadcrumps = fileserver.breadcrumps;
		const parent = fileserver.parent;
		const readme = fileserver.readme;
		const current_document = fileserver.current_document;
		let path = params.index;

		return { props: { project, current_document, params, readme, path, documents, breadcrumps, parent } };
	}
</script>

<script>
    import { invalidate } from '$app/navigation';
	import { page } from '$app/stores';

	import Uploader from "$lib/filemanager/Uploader.svelte";
	import CreateFolder from "$lib/filemanager/CreateFolder.svelte";
	import RenderDocument from "$lib/filemanager/RenderDocument.svelte";
	import TableHeaderActions from "$lib/filemanager/TableHeaderActions.svelte";
	import TableBreadcrumps from "$lib/filemanager/TableBreadcrumps.svelte";
	import TableRow from "$lib/filemanager/TableRow.svelte";
	import ProjectTopMenu from "$lib/filemanager/ProjectTopMenu.svelte";
	import TableMultipleSelections from "$lib/filemanager/TableMultipleSelections.svelte";

	export let project;
	export let documents;
	export let breadcrumps;
	export let parent;
	export let readme;
	export let path;
	export let current_document;

	let create_folder_input;
	let qwickActionsMenuItemId = null;
	let show_upload = false;
	let show_create_folder = false;
	let show_multiple_selection = false;
	let selections = [];
	
	let isNotShared = $page.url.search != '?share=true';
	let parent_document_url = `/projects/${$page.params.uid}/documents/${parent.name}`;
	
	if (! isNotShared) {
        parent_document_url = parent_document_url + '?share=true'
    }

	async function create_folder() {
		// ${$page.params.index}${$page.params.index} create_folder_input
		await apiDirect(`fileserver/documents/${project.uuid}/create-folder/${$page.params.index}`, {
			folder_path: create_folder_input,
		}, 'POST')

		invalidate(`/projects/${project.uuid}/documents/${$page.params.index}`)
	}
</script>
	
<article class="max-w-6xl mx-auto py-12">
	<ProjectTopMenu project={project} isNotShared={isNotShared} />

	<section class="bg-white shadow rounded container mx-auto">
		<div class="card p-0">
			<div class="relative w-full">
				{#if isNotShared}
					<!-- Actions -->
					<TableHeaderActions bind:documents bind:show_upload bind:show_create_folder hideLabel class="min-w-full md:min-w-0 mt-0 block w-full px-0.5 border-0 border-b-2 border-gray-200 focus:ring-0 focus:border-black flex-1" />
				{/if}

				<!-- Breadcrumps -->
				<TableBreadcrumps bind:breadcrumps />
			</div>

			{#if isNotShared}
				{#if show_create_folder}
				<!-- CreateFolder -->
					<CreateFolder bind:value={create_folder_input} on:submit={create_folder} />
				{/if}
				
				<!-- Uploader -->
				{#if show_upload}
					<Uploader project={project} />
				{/if}
				
				<!-- TableMultipleSelections -->
				{#if show_multiple_selection}
					<TableMultipleSelections bind:checkedRow={selections} {documents} />
				{/if}
			{/if}


			<table tabindex="0" class="table w-full">
				<thead>
					<tr>
						<th class="checkbox-column py-2 px-3">
						<th class="pt-4 text-xs font-semibold text-gray-500 uppercase text-left current-column sortable-column">
							<span>Nom</span>
						</th>

						<th class="pt-4 text-xs font-semibold text-gray-500 uppercase text-left current-column sortable-column">
							<span>Taille</span>
						</th>
						
						<th class="pt-4 text-xs font-semibold text-gray-500 uppercase text-left current-column sortable-column">
							<span>Dernière modification</span>
						</th>
						
						<th class="actions-column"></th>
					</tr>
				</thead>

				<tbody class="divide-y">
					<!-- Back button -->
					<tr class="hover:bg-gray-100">
						<td class="w-12" />
						
						<td class="py-3">
							<a href={parent_document_url} class="flex items-center cursor-pointer group text-blue-500 hover:underline">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mr-2" viewBox="0 0 20 20" fill="currentColor">
									<path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
								</svg>
								<span class="font-semibold">..</span>
							</a>
						</td> 
						<td colspan="3" />
					</tr>

					<!-- List documents -->
					{#each documents as document, document_index}
						<TableRow path={path} isNotShared={isNotShared} bind:bindGroup={selections} bind:hasOneSelectedItem={show_multiple_selection} project={project} document={document} document_index={document_index} bind:qwickActionsMenuItemId />
					{/each}
				</tbody>
			</table>
		</div>
	</section>

	{#if readme.exist || current_document == "File"}
		<section class="bg-white shadow rounded container mx-auto mt-4">
			<RenderDocument isNotShared={isNotShared} project={project} path={readme.exist ? readme.path : path} />
		</section>
	{/if}
</article>