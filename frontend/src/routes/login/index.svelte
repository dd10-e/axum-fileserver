<svelte:head>
	<title>Se connecter</title>
</svelte:head>

<script context="module">
	export async function load({ session }) {
		if (session.user) {
			return {
				status: 302,
				redirect: '/projects'
			};
		}

		return {};
	}
</script>

<script>
	import { enhance } from '$lib/form';
	import logo from '$lib/header/logo.svg';

	let isSending = false;
	let error;
</script>

<article class="max-w-6xl mx-auto py-12">
	<div class="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8">
		<div class="sm:mx-auto sm:w-full sm:max-w-md">
			<img class="w-64 mx-auto" src={logo} alt=""/>
			<h2 class="mt-6 text-center text-xl font-bold uppercase text-gray-900">
				Connectez-vous à votre compte
			</h2>
		</div>

		<div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
			<div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
				<form 
					class="space-y-6" 
					action="/login.json" 
					method="POST"
					use:enhance={{
						pending: async (res) => {
							isSending = true;
							error = null;
						},
						result: async (res) => {
							window.location = "/"
						},
						error: async(res) => {
							let error_response = await res.json();

							isSending = false;
							error = error_response.error;
						}
					}}
				>
					<div>
						<label for="email" class="block text-sm font-medium text-gray-700">
							Adresse email
						</label>
						<div class="mt-1">
							<input id="email" name="email" type="text" autocomplete="email" required
								class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm">
						</div>
					</div>

					<div>
						<label for="password" class="block text-sm font-medium text-gray-700">
							Mot de passe
						</label>
						<div class="mt-1">
							<input id="password" name="password" type="password" autocomplete="current-password" required
								class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm">
						</div>
					</div>

					<!-- <div class="flex items-center justify-between">
						<div class="flex items-center">
							<input id="remember-me" name="remember-me" type="checkbox"
								class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded">
							<label for="remember-me" class="ml-2 block text-sm text-gray-900">
								Se souvenir de moi
							</label>
						</div>
					</div> -->

					{#if error}
						<div>
							<p class="text-red-500">{error}</p>
						</div>
					{/if}

					<div>
						<button type="submit"
							class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
							{isSending ? `Connexion` : `Se connecter`}
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
</article>
