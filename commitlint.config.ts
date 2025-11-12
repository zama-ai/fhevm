import type { UserConfig } from "@commitlint/types";

const Configuration: UserConfig = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      ["build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test"],
    ],
    "scope-enum": [
      2,
      "always",
      [
        "coprocessor",
        "host-contracts",
        "gateway-contracts",
        "protocol-contracts",
        "contracts",
        "library-solidity",
        "kms-connector",
        "sdk",
        "test-suite",
        "charts",
        "common",
      ],
    ],
  },
};

export default Configuration;
