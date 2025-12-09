import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    outDir: 'dist',
    rollupOptions: {
      input: {
        main: './index.html',
      },
    },
  },
  optimizeDeps: {
    exclude: ['../wasm/tdf_wasm.js'], // Exclude WASM from pre-bundling
  },
  server: {
    fs: {
      allow: ['..'], // Allow accessing parent directories for WASM
    },
  },
});
