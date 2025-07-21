/// <reference types="vitest" />
import { defineConfig } from 'vitest/config'
const isCI = process.env.CI === "true";

export default defineConfig({
    build: {
        minify: 'terser',

        terserOptions: {
            format: {
                comments: 'all',
                preserve_annotations: true,
            },

        },

    },
    test: {
        setupFiles: [],
        reporters: ['verbose'],
        browser: {
            enabled: false
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
            include: [
                'build/**/*'
            ],
        },

    }
})
