module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [2, "always", ["ci", "chore", "docs", "ticket", "feat", "fix", "perf", "refactor", "revert", "style"]],
  },
};
