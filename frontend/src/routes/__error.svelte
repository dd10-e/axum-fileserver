<script context="module">
	export function load({ error, status }) {
    console.log(error)
		return {
			props: {
        error: error,
				status: status
			}
		};
	}
</script>

<script>
	export let title;
	export let status;
	export let error;
</script>

<article class="max-w-6xl mx-auto py-12">
  <p class="text-sm font-semibold text-blue-600 uppercase tracking-wide">Error {status}</p>
  <h1 class="mt-2 text-4xl font-extrabold text-gray-900 tracking-tight sm:text-5xl">{error.name}</h1>
  <p class="mt-2 text-base text-gray-500">{error.message}</p>
      <div class="mt-6">
          <a href="/projects" class="text-base font-medium text-blue-600 hover:text-blue-500">
              Voir la liste des projets <span aria-hidden="true"> &rarr;</span>
          </a>

          <div class="prose max-w-none">
            <h3 class="mt-8">Stack : </h3>
  
            <pre>
              <code>
                {error.stack}
              </code>
            </pre>
          </div>
      </div>
</article>
  