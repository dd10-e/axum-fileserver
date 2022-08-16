<script context="module">
	export function load({ url, session }) {    
    let isNotShared = url.search != '?share=true';

    if (! session.user && isNotShared) {
      return {
          status: 302,
          redirect: '/login'
        };
    }

    return {};
	}
</script>

<script>
  import { navigating } from '$app/stores';
	import { fade } from 'svelte/transition';

  import Header from "$lib/header/Header.svelte";
  import "../app.css";

  let isLoading = false;
</script>

<svelte:window
    on:sveltekit:navigation-start={() => {
      isLoading = true;
    }}
    on:sveltekit:navigation-end={() => {
      isLoading = false;
    }}
/>

<Header />

{#if isLoading}
<div transition:fade={{ delay: 1000 }} class="fixed bottom-0 left-0 p-12">
  <svg class="text-blue-500 scale-[0.3]" width="135" height="140" viewBox="0 0 135 140" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><rect y="10" width="15" height="120" rx="6"><animate attributeName="height" begin="0.5s" dur="1s" values="120;110;100;90;80;70;60;50;40;140;120" calcMode="linear" repeatCount="indefinite"/><animate attributeName="y" begin="0.5s" dur="1s" values="10;15;20;25;30;35;40;45;50;0;10" calcMode="linear" repeatCount="indefinite"/></rect><rect x="30" y="10" width="15" height="120" rx="6"><animate attributeName="height" begin="0.25s" dur="1s" values="120;110;100;90;80;70;60;50;40;140;120" calcMode="linear" repeatCount="indefinite"/><animate attributeName="y" begin="0.25s" dur="1s" values="10;15;20;25;30;35;40;45;50;0;10" calcMode="linear" repeatCount="indefinite"/></rect><rect x="60" width="15" height="140" rx="6"><animate attributeName="height" begin="0s" dur="1s" values="120;110;100;90;80;70;60;50;40;140;120" calcMode="linear" repeatCount="indefinite"/><animate attributeName="y" begin="0s" dur="1s" values="10;15;20;25;30;35;40;45;50;0;10" calcMode="linear" repeatCount="indefinite"/></rect><rect x="90" y="10" width="15" height="120" rx="6"><animate attributeName="height" begin="0.25s" dur="1s" values="120;110;100;90;80;70;60;50;40;140;120" calcMode="linear" repeatCount="indefinite"/><animate attributeName="y" begin="0.25s" dur="1s" values="10;15;20;25;30;35;40;45;50;0;10" calcMode="linear" repeatCount="indefinite"/></rect><rect x="120" y="10" width="15" height="120" rx="6"><animate attributeName="height" begin="0.5s" dur="1s" values="120;110;100;90;80;70;60;50;40;140;120" calcMode="linear" repeatCount="indefinite"/><animate attributeName="y" begin="0.5s" dur="1s" values="10;15;20;25;30;35;40;45;50;0;10" calcMode="linear" repeatCount="indefinite"/></rect></svg>
</div>
{/if}

<main>
  <slot />
</main>