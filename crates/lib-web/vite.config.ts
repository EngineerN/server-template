import path from 'node:path'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import license from 'rollup-plugin-license'

// https://vitejs.dev/config/
export default defineConfig({
  base: '/web',
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  },
  build: {
    rollupOptions: {
      plugins: [
        license({
          thirdParty: {
            includePrivate: false,
            allow: {
              test: (dependency) => {
                // Return false for unlicensed dependencies.
                if (!dependency.license) {
                  return false
                }

                // Allow MIT and Apache-2.0 licenses.
                return ['MIT', 'Apache-2.0'].includes(dependency.license)
              },
              failOnUnlicensed: true,
              failOnViolation: true
            },
            output: path.resolve(__dirname, './dist/LICENSE.txt')
          }
        })
      ]
    }
  },
  esbuild: {
    banner: '/*! licenses: /web/LICENSE.txt */',
    legalComments: 'none'
  },
  server: {
    host: "127.0.0.1",
    port: 4000,
    strictPort: true
  }
})
