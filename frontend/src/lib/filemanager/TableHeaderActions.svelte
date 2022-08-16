<script>
    import Search from "svelte-search";

	export let show_upload = false;
	export let show_create_folder = false;

    export let documents;
    let all_documents = documents;

    let search = "";

    function showUploaderInput() {
		show_upload = !show_upload
        show_create_folder = false

	}

    function showCreateFolderInput() {
		show_create_folder = !show_create_folder
        show_upload = false
	}

    async function handleSearch() {
        documents = all_documents;
        documents = documents.filter(document => document.name.toLowerCase().indexOf(search.toLowerCase()) >= 0);
    }

    async function handleClearSearch() {
        documents = all_documents;
    }
</script>

<div class="px-6 py-4 flex flex-wrap items-center justify-between space-y-2 md:space-y-0 md:space-x-2">
    <div class="flex-1">
        <Search bind:value={search} hideLabel debounce={500} on:type={handleSearch} on:clear={handleClearSearch} type="text" name="search" id="search" class="min-w-full md:min-w-0 mt-0 block w-full px-0.5 border-0 border-b-2 border-gray-200 focus:ring-0 focus:border-black" placeholder="Rechercher un document..." />
    </div>

    <!-- <input 
        type="text" 
        class="min-w-full md:min-w-0 mt-0 block w-full px-0.5 border-0 border-b-2 border-gray-200 focus:ring-0 focus:border-black flex-1" 
        placeholder="Rechercher..."
    > -->

    <button on:click={showCreateFolderInput} aria-label="Ouvrir" class="md:min-w-0 btn-flat bg-gray-100 hover:bg-gray-200 h-12 px-4 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1 text-gray-900" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
        </svg>
        <span>Nouveau dossier</span>
    </button>

    <button on:click={showUploaderInput} class="md:min-w-0 btn-flat bg-gray-100 hover:bg-gray-200 h-12 px-4 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1 text-gray-900" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
        </svg>
        <span>Téléverser</span>
    </button>

    <!-- <button class="md:min-w-0 btn-flat bg-gray-100 hover:bg-gray-200 h-12 px-4 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-600" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
          </svg>
    </button> -->
</div>

<style>
    [data-svelte-search] {
        flex: 1;
    }
</style>