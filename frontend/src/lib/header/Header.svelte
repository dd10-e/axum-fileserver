<script>
	import { page, session } from '$app/stores';
	import logo from './logo.svg';

	let isNotShared = $page.url.search != '?share=true';
</script>

<header class="bg-white shadow">
	  <div
		class="max-w-6xl mx-auto flex justify-between items-center relative z-10 space-x-2 sm:space-x-3"
	  >
	  	<div class="shrink-0">
			<a href="/">
				<img class="w-44" src={logo} alt=""/>
			</a>
		</div>
  
		{#if isNotShared}
			 <!-- content here -->
			 <div class="space-x-8 md:space-x-12 p-4 sm:p-7 flex items-center text-sm font-semibold">
				 <a
					 href="/users"
					 class="text-gray-700">
					 Utilisateurs
				 </a>
	 
				 <a class="border border-gray-300 hover:border-gray-400 hover:border-blue-700 hover:text-blue-700 rounded px-4 py-2" href="/projects">
					 Liste des projets
				 </a>
	 
				 {#if $session.user}
					 <a
						 href="/account"
						 class="text-gray-700">
						 {$session.user.email}
					 </a>
				 {/if}
			 </div>
		{/if}
	  </div>
</header>
