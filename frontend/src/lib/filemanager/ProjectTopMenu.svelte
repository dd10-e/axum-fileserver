<script>
    import { getStores, navigating, page, session } from '$app/stores';
    
    export let project;
    export let isNotShared;

    let isSettingsPage = $page.url.pathname === `/projects/${project.uuid}/settings`;
</script>

<section class="container mx-auto flex justify-between flex-wrap mb-3">
    <nav class="flex" aria-label="Breadcrumb">
        <ol role="list" class="flex items-center space-x-2">
            <li>
                <div class="flex items-center">
                    <p class="text-sm font-semibold text-gray-500 hover:text-gray-700 whitespace-nowrap">Home</p>
                </div>
            </li>

            <li>
                <div class="flex items-center">
                    <svg class="flex-shrink-0 h-5 w-5 text-gray-300" xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                        viewBox="0 0 20 20" aria-hidden="true">
                        <path d="M5.555 17.776l8-16 .894.448-8 16-.894-.448z" />
                    </svg>
                    <a href="/projects/{project.uuid}/documents" class="ml-2 text-sm font-medium text-gray-500 hover:text-gray-700 whitespace-nowrap" aria-current="page">
                        {project.name}
                    </a>
                </div>
            </li>
        </ol>
    </nav>  

    {#if isNotShared}
         <div class="flex space-x-2">
             <a target="_blank" href="/projects/{project.uuid}/documents?share=true" class="flex items-center text-gray-800 text-xs space-x-2 px-2 py-1 hover:text-blue-600">
                 <p>Partager</p>
                 <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                 </svg>
             </a>
     
             {#if isSettingsPage}
                  <a href="/projects/{project.uuid}/documents" class="flex items-center text-gray-800 text-xs space-x-2 px-3 py-1 hover:text-blue-600">
                      <p>Documents</p>
                      <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="0.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                      </svg>
                  </a>
             {:else}
                 <a href="/projects/{project.uuid}/settings" class="flex items-center text-gray-800 text-xs space-x-2 px-2 py-1 hover:text-blue-600">
                     <p>Paramètres</p>
                     <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                     </svg>
                 </a>
             {/if}
         </div>
    {/if}
</section>