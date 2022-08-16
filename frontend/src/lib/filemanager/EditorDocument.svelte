<style>
	@import 'quill/dist/quill.snow.css';
</style>

<script>
	import { onMount } from 'svelte';

	import { apiDirect, baseApi } from '$lib/services/_server.js';
	
	export let project;
	export let isEditable;
	export let toolbarOptions = [
		[{ header: 1 }, { header: 2 }, "blockquote", "link", "image"],
		["bold", "italic", "underline", "strike"],
		[{ list: "ordered" }, { list: "ordered" }],
		[{ align: [] }],
		["clean"]
	];
	
	let editor;

	onMount(async () => {
		const { default: Quill } = await import("quill");

		let quill = new Quill(editor, {
			modules: {
				toolbar: toolbarOptions
			},
			theme: "snow",
			placeholder: "Vous pouvez rédiger votre document ici..."
		});
	});

	const res = fetch(`${baseApi}/fileserver/documents/${project.uuid}/serve-file/Index.html`)
		.then(response => response.text())
		.then((response) => {
			editor.innerHTML = response
        })


	async function submit() {
		await fetch(`${baseApi}/fileserver/documents/${project.uuid}/edit-file/Index.html`, {
			method: 'POST',
			body: JSON.stringify({
				file_content: editor.firstChild.innerHTML,
			}),
			headers: {
				'Content-type': 'application/json; charset=UTF-8'
			}
		})

		isEditable = false;
	}
</script>

<div>
	<div bind:this={editor} />
</div>

<button
	on:click={submit}
	type="submit"
	class="mt-8 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
	{`Mettre à jour`}
</button>