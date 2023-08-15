import staticAdapter from "@sveltejs/adapter-static";
import {vitePreprocess} from "@sveltejs/kit/vite";
/** @type {import('@sveltejs/kit').Config}*/
const config = {
  preprocess: [vitePreprocess()],
  kit: {
    adapter: staticAdapter(),
    alias: {
      $components: "src/lib/components",
      "$components/*": "src/lib/components/*"
    }
  },
  shadcn: {
    componentPath: './src/lib/components/ui'
  }
};
export default config;
