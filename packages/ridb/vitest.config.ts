/// <reference types="vitest" />
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";


const isCI = process.env.CI === "true";

export default defineConfig({
    plugins: [
      //  wasm(),
      ],
    build: {
        minify: 'terser',
        terserOptions: {
            format: {
                comments: 'all',
                preserve_annotations: true,
            },
        }
    },
    test: {
        setupFiles: ['./tests/setup.ts'],
        reporters: ['verbose'],
        browser: {
            provider: 'webdriverio',
            enabled: true,
            headless: true,
            instances: [
                {
                    browser: 'chrome',
                }
            ],
        },
        coverage: {
            provider: 'istanbul',
            reporter: isCI ? ['json-summary'] : ['json-summary', "html"],
            thresholds: {
                branches: 100,
                functions: 100,
                lines: 100,
                statements: 100
            },
           
        },
    }
})
