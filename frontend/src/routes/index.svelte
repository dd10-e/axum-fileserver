<svelte:head>
	<title>Home</title>
</svelte:head>

<script context="module">
  export async function load() {
    return {
      status: 302,
      redirect: `/projects`,
    }
  }
</script>