<script>
    import { createEventDispatcher } from 'svelte';

    export let name;
    export let value = '';

    let menuOpen = false;

    const dispatch = createEventDispatcher();

    function toggleMenu() {
        menuOpen = !menuOpen;
    }

	const add_folder = () => {
        dispatch('submit')

        menuOpen = false
        value = ''
    };
</script>

<span class="relative">
    <button on:click={toggleMenu} aria-label="Ouvrir" class="md:min-w-0 btn-flat bg-gray-100 hover:bg-gray-200 h-12 px-4 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1 text-gray-900" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
        </svg>
        <span>{name}</span>
    </button>
    
    {#if menuOpen}
        <div class="absolute top-full right-0">
            <div>
                <div class="shadow-xl p-2 bg-white rounded-md flex">
                    <input
                        bind:value={value}
                        type="text" 
                        class="w-64 block w-full px-0.5 border-0 border-b-2 border-gray-200 focus:ring-0 focus:border-black flex-1" 
                        placeholder="Créer un dossier..."
                    >
                    <button  on:click={add_folder} class="bg-gray-100 text-left hover:bg-gray-200 px-2 py-1">Ajouter</button>
                </div>
            </div>
        </div>
    {/if}
</span>