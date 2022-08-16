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

<div>
    <div class="px-8 py-3 bg-gray-100 rounded-md flex">
        <input
            bind:value={value}
            type="text" 
            class="block w-full px-4 border-0 border-b-2 border-gray-200 focus:ring-0 focus:border-black flex-1" 
            placeholder="Créer un dossier..."
        >
        <button  on:click={add_folder} class="bg-gray-300 text-left hover:bg-gray-200 px-2 py-1">Ajouter</button>
    </div>
</div>