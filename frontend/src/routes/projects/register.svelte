<article class="max-w-6xl mx-auto py-12">
    <section class="bg-white shadow rounded container mx-auto">
        <div class="p-8">
            <div class="max-w-lg space-y-4">
                <!-- General informations -->
                <div>
                    <label for="project-name" class="block text-sm font-medium text-gray-700">
                        Nom du projet
                    </label>
    
                    <div class="mt-1">
                        <input type="text" name="project-name" id="project-name" class="block w-full shadow-sm focus:ring-sky-500 focus:border-sky-500 sm:text-sm border-gray-300 rounded-md" value="Cabinet dentaire" />
                    </div>
                </div>
      
                <div>
                    <label for="description" class="block text-sm font-medium text-gray-700">
                        Description
                    </label>
    
                    <div class="mt-1">
                        <textarea id="description" name="description" rows="3" class="block w-full shadow-sm focus:ring-sky-500 focus:border-sky-500 sm:text-sm border border-gray-300 rounded-md"></textarea>
                    </div>
                </div>
    
                <div>
                    <label for="tags" class="block text-sm font-medium text-gray-700">
                        Tags
                    </label>
                    <input type="text" name="tags" id="tags" class="mt-1 block w-full shadow-sm focus:ring-sky-500 focus:border-sky-500 sm:text-sm border-gray-300 rounded-md" />
                </div>
            </div>

            <div class="mt-5">
                <button type="button" class="inline-flex items-center px-5 py-2 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                    Ajouter un projet
                </button>
            </div>
        </div>
    </section>
</article>