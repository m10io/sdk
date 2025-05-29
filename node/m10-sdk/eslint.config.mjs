// @ts-check

import eslint from "@eslint/js";
import * as importPlugin from "eslint-plugin-import";
import simpleImportSort from "eslint-plugin-simple-import-sort";
import tseslint from "typescript-eslint";
import stylisticJs from "@stylistic/eslint-plugin-js";
import stylisticTs from "@stylistic/eslint-plugin-ts";
import stylisticJsx from "@stylistic/eslint-plugin-jsx";
import unusedImports from "eslint-plugin-unused-imports";
import { createTypeScriptImportResolver } from "eslint-import-resolver-typescript";

export default tseslint.config(
    // core
    eslint.configs.recommended,
    tseslint.configs.recommended,
    {
        plugins: {
            "@stylistic/js": stylisticJs,
            "@stylistic/ts": stylisticTs,
            "@stylistic/jsx": stylisticJsx,
        },
        rules: {
            // Place custom rules here
            // use stylistic/{js/ts/jsx} instead of core eslint deprecated rules

            "@stylistic/js/indent": ["error", 4, { SwitchCase: 1 }],
            "@stylistic/js/max-len": ["warn", { code: 160 }],

            // already in recommended
            // "no-cond-assign": "warn",

            "no-multiple-empty-lines": ["error", { max: 3 }],
            "no-trailing-spaces": "error",
            // Disable core rules in favor of TypeScript-specific ones
            semi: "off",
            "@stylistic/js/comma-dangle": "off",
            quotes: ["error", "double"],
            "no-console": "warn",

            // Turn off legacy/deprecated rules
            "interface-name": "off",
            "one-line": "off",
            "member-ordering": "off",
            "object-literal-sort-keys": "off",

            "eol-last": ["error", "always"],
            "no-shadow": "off",
            "@stylistic/js/object-curly-spacing": ["warn", "always"],
            "array-bracket-spacing": ["warn", "never"],

            // @typescript-eslint rules
            "@typescript-eslint/explicit-member-accessibility": "error",
            "@typescript-eslint/explicit-function-return-type": [
                "error",
                { allowExpressions: true },
            ],
            "@typescript-eslint/no-inferrable-types": "off",
            "@typescript-eslint/no-magic-numbers": [
                "warn",
                {
                    ignoreReadonlyClassProperties: true,
                    ignoreEnums: true,
                    ignore: [0],
                },
            ],
            // For array-type, enforcing the “T[]” style.
            "@typescript-eslint/array-type": ["error", { default: "array" }],
            "@stylistic/ts/semi": "error",
            "@stylistic/ts/comma-dangle": [
                "error",
                {
                    enums: "always-multiline",
                    generics: "always-multiline",
                    tuples: "always-multiline",
                    arrays: "always-multiline",
                    objects: "always-multiline",
                    imports: "always-multiline",
                    exports: "always-multiline",
                    functions: "always-multiline",
                },
            ],
            "@stylistic/ts/member-delimiter-style": [
                "error",
                {
                    singleline: {
                        delimiter: "comma",
                        requireLast: false,
                    },
                },
            ],
            "@typescript-eslint/no-shadow": ["warn", { ignoreTypeValueShadow: true }],
            "@typescript-eslint/consistent-type-imports": [
                "warn",
                { prefer: "type-imports" },
            ],
        },
    },

    // import plugins
    {
        ...importPlugin.flatConfigs?.recommended,
        files: ["src/**/*.{js,mjs,cjs,ts,tsx}"],
        languageOptions: {
            ecmaVersion: "latest",
            sourceType: "module",
        },
        settings: {
            "import/resolver-next": [
                createTypeScriptImportResolver({
                    // always try to resolve types under `<root>@types` directory even it doesn't contain any source code, like `@types/unist`
                    alwaysTryTypes: true,

                    // use <root>/path/to/folder/tsconfig.json
                    project: "./tsconfig.json",
                }),
            ],
        },
        plugins: {
            import: importPlugin.flatConfigs?.recommended.plugins.import,
            "simple-import-sort": simpleImportSort,
            "unused-imports": unusedImports,
        },
        rules: {
            "no-unused-vars": "off",
            "import/no-dynamic-require": "warn",
            "import/no-nodejs-modules": "warn",

            "simple-import-sort/imports": [
                "error",
                {
                    groups: [
                        // Packages come first.
                        ["^@?\\w"],
                        // Parent imports. Put `..` last.
                        ["^\\.\\.(?!/?$)", "^\\.\\./?$"],
                        // Other relative imports. Put same-folder imports and `.` last.
                        ["^\\./(?=.*/)(?!/?$)", "^\\.(?!/?$)", "^\\./?$"],
                    ],
                },
            ],
            "simple-import-sort/exports": "error",

            "@typescript-eslint/no-unused-vars": "off",
            "unused-imports/no-unused-imports": "error",
            "unused-imports/no-unused-vars": [
                "warn",
                {
                    vars: "all",
                    varsIgnorePattern: "^_",
                    args: "after-used",
                    argsIgnorePattern: "^_",
                    caughtErrorsIgnorePattern: "^_|error",
                },
            ],
        },
    },
    // Disable `no-unused-expressions` in test files (for Chai assertions)
    {
        files: ["tests/**/*.test.ts", "tests/**/*.spec.ts"],
        rules: {
            "@typescript-eslint/no-unused-expressions": "off",
        },
    },

    // ignore patterns
    {
        ignores: [
            "node_modules/",
            "src/protobufs/",
            "out",
            "dist",
            "postbuild.cjs",
            "**/*.d.ts",
            "*.config.js",
            "*.config.mjs",
            "*.config.cjs",
        ],
    },
);
