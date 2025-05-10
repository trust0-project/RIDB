import { defineConfig } from "eslint/config";
import js from "@eslint/js";
import tseslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import { disabledRules } from "../../eslint.base.mjs";
export default defineConfig([
	{ 
		ignores: ["dist", "node_modules", ".eslintrc.js"]
	},
	// Base JS config for all files
	{
		files: ["src/**/*.{js,mjs,cjs,ts}"],
		plugins: { js },
		rules: {
			...js.configs.recommended.rules,
			...disabledRules,
			"@typescript-eslint/no-require-imports": "off",
		},
	},
	// TypeScript specific configuration
	{
		files: ["src/**/*.ts"],
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				ecmaVersion: "latest",
				sourceType: "module",
				project: "./tsconfig.json"
			},
		},
		plugins: {
			"@typescript-eslint": tseslint
		},
		rules: {
			...tseslint.configs.recommended.rules,
			...disabledRules,
			"@typescript-eslint/no-require-imports": "off",
		},
	}
]);