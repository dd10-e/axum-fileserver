<script>
    export let checkedRow;
    export let documents;

    let show = false;
    
    $: checked = checkedRow.length !== 0;

    function onChange() {
		if (checkedRow.length === 0) {
            // TODO
            let documents_ids = documents.map((document, index) => {
                return index;
            });

            checkedRow = documents_ids;
        } else {
            checkedRow = [];
        }
	}

    function bulkMove() {
        console.log(checkedRow)
    }

    function bulkDelete() {
        console.log(checkedRow)
    }
</script>

<div>
    <div class="px-5 py-3 bg-gray-100 flex items-center space-x-4">
        <label for="select-all-table-row" class="flex items-center space-x-2">
            <input
                on:change={onChange}
                {checked}
                id="select-all-table-row"
                type="checkbox"
            />
            <p class="text-gray-800 text-sm">{checkedRow.length} selections</p>
        </label>

        <span class="relative z-0 inline-flex shadow-sm rounded-md">
                <button 
                    on:click={() => show = !show} 
                    type="button"
                    class="relative inline-flex items-center rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:z-10 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                >
                    <span class="px-4 py-2">Appliquer une action</span>

                    <span class="relative inline-flex items-center px-2 py-2 rounded-r-md text-sm font-medium">
                        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
                            aria-hidden="true">
                            <path fill-rule="evenodd"
                                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                clip-rule="evenodd" />
                        </svg>
                    </span>
                </button>

                {#if show}
                    <span class="-ml-px relative block">
                        <div class="origin-top-right top-full absolute right-0 mt-2 -mr-1 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none"
                            role="menu" aria-orientation="vertical" aria-labelledby="option-menu-button" tabindex="-1">
                            <div class="py-1" role="none">
                                <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
                                <button on:click={bulkMove} class="text-gray-700 block w-full text-left px-4 py-2 text-sm hover:bg-gray-100" role="menuitem" tabindex="-1"
                                    id="option-menu-item-0">
                                    Déplacer
                                </button>
    
                                <button on:click={bulkDelete} class="text-gray-700 block w-full text-left px-4 py-2 text-sm hover:bg-gray-100" role="menuitem" tabindex="-1"
                                    id="option-menu-item-2">
                                    Supprimer
                                </button>
                            </div>
                        </div>
                    </span>
                {/if}
        </span>  
    </div>
</div>