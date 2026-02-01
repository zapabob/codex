import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { readFileSync } from "fs";
import { resolve } from "path";

// Read version from package.json
const packageJson = JSON.parse(
  readFileSync(resolve(__dirname, "package.json"), "utf-8"),
);
const version = packageJson.version;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Define global constants
  define: {
    __APP_VERSION__: JSON.stringify(version),
    __APP_NAME__: JSON.stringify(packageJson.name),
  },

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
