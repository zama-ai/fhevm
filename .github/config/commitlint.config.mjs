import { RuleConfigSeverity } from "@commitlint/types";

export default {
  extends: ["@commitlint/config-conventional"],
  rules: {
    // Enforce that type is always present
    "type-empty": [RuleConfigSeverity.Error, "never"],
    
    // Allow sentence-case or lower-case (prevents all-caps or PascalCase)
    // This allows: "add feature [API-123]" or "fix TransactionService bug"
    // But rejects: "ADD FEATURE" or "AddFeature"
    "subject-case": [
      RuleConfigSeverity.Error,
      "never",
      ["upper-case", "pascal-case", "start-case"]
    ],
  },
};


