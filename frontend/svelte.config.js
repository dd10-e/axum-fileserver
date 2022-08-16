import preprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-node";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  server: {
    https: false,
  },
  kit: {
    adapter: adapter({ out: 'build' }),
    files: {
      assets: "assets",
    },
		methodOverride: {
			allowed: ['PUT', 'PATCH', 'DELETE']
		},
    vite: {
      define: {
        'process.env': process.env,
      },
      optimizeDeps: {
        exclude: ["dayjs"],
      },
    },
  },

  preprocess: [preprocess({})],
};

export default config;