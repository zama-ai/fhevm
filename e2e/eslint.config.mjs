import eslint from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default [
  {
    languageOptions: {
      globals: globals.node,
    },
    linterOptions: {
      reportUnusedDisableDirectives: "off",
    },
    ignores: ["abi/", "artifacts/", "cache/", "res/", "types/*"],
  },
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
];
