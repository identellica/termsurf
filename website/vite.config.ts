import { defineConfig } from "vite";
import tsConfigPaths from "vite-tsconfig-paths";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";

export default defineConfig({
  server: {
    port: 3000,
  },
  build: {
    outDir: "dist/client",
  },
  plugins: [
    tsConfigPaths(),
    TanStackRouterVite({ autoCodeSplitting: true }),
    react(),
    tailwindcss(),
  ],
});
