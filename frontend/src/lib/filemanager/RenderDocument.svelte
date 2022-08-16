<script>
	import { onMount } from 'svelte';
	import { apiDirect, baseApi } from '$lib/services/_server.js';

	import EditorDocument from "$lib/filemanager/EditorDocument.svelte"
	
	export let path;
	export let project;
	export let isNotShared;

	let isEditable = false;
</script>

<section class="p-4">
	<div class="flex justify-between">
		<p class="text-sm">{path}</p>

		{#if isNotShared}
			 <!-- content here -->
			<div class="flex flex-wrap space-x-1">
				<a href={`${baseApi}/fileserver/documents/${project.uuid}/download-file/${path}`} download class="flex items-center text-gray-800 text-xs px-2 py-1 hover:text-blue-600">
					<!-- <span>Télécharger</span> -->
					<svg class="h-6 w-6 ml-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
					</svg>
				</a>

				<button on:click={() => isEditable = !isEditable} class="flex items-center text-gray-800 text-xs px-2 py-1 hover:text-blue-600">
					<!-- <span>Éditer</span> -->
					<svg class="h-6 w-6 ml-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10.3125 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
				</button>
			</div>
		{/if}
	</div>

	<div class="p-4 mt-4 border rounded-md">
		{#if isEditable === false}
			<embed
				title="Document viewver"
				src={`${baseApi}/fileserver/documents/${project.uuid}/serve-file/${path}`}
				class="aspect-video w-full"
			>
		{:else}
			<EditorDocument bind:isEditable project={project} />
		{/if}
	</div>
</section>