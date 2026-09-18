module.exports = (async () => {
  const [{ default: eslint }, { default: tseslint }, { default: prettier }, { default: globals }] = await Promise.all([
    import("@eslint/js"),
    import("typescript-eslint"),
    import("eslint-config-prettier"),
    import("globals"),
  ]);

  return tseslint.config(
    {
      ignores: [
        "fhevmTemp/**",
        "tmp/**",
        ".coverage_artifacts/**",
        ".coverage_cache/**",
        ".coverage_contracts/**",
        "artifacts/**",
        "build/**",
        "cache/**",
        "coverage/**",
        "dist/**",
        "node_modules/**",
        "types/**",
        "*.env",
        "*.log",
        "coverage.json",
      ],
    },
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    prettier,
    {
      files: ["**/*.js"],
      languageOptions: {
        globals: {
          ...globals.node,
        },
      },
    },
    {
      files: ["**/*.ts"],
      languageOptions: {
        parserOptions: {
          project: "./tsconfig.json",
        },
      },
      rules: {
        "@typescript-eslint/no-floating-promises": ["error", { ignoreIIFE: true, ignoreVoid: true }],
        "@typescript-eslint/no-inferrable-types": "off",
        "@typescript-eslint/no-unused-vars": ["error", { argsIgnorePattern: "_", varsIgnorePattern: "_" }],
      },
    },
  );
})();
