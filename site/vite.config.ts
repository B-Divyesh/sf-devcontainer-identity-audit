import { resolve } from "node:path";
import { defineConfig } from "vite";
import { injectBuildIdentity } from "./build-identity";

export default defineConfig({
  root: resolve(import.meta.dirname),
  publicDir: "public",
  plugins: [{
    name: "mount-identity-audit-build-identity",
    transformIndexHtml: injectBuildIdentity
  }],
  build: {
    outDir: resolve(import.meta.dirname, "../dist/site"),
    emptyOutDir: true,
    target: "es2022",
    cssCodeSplit: true,
    rollupOptions: {
      input: {
        notFound: resolve(import.meta.dirname, "404.html"),
        main: resolve(import.meta.dirname, "index.html"),
        demo: resolve(import.meta.dirname, "demo/index.html"),
        privacy: resolve(import.meta.dirname, "privacy/index.html"),
        terms: resolve(import.meta.dirname, "terms/index.html")
      }
    }
  }
});
