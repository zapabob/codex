import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },

  // Mark problematic Babylon.js modules as external
  optimizeDeps: {
    exclude: ["@babylonjs/serializers", "@babylonjs/gui-editor"],
  },
  build: {
    rollupOptions: {
      external: [
        "@babylonjs/core/Materials/PBR/openpbrMaterial.js",
        "@babylonjs/core/Materials/Textures/textureMerger.js",
        "@babylonjs/gui-editor",
        "@babylonjs/serializers",
      ],
    },
  },
}));
