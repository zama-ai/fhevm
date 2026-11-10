#!/usr/bin/env node
// Parse workflow_dispatch `overrides` JSON into GITHUB_OUTPUT.
// Env: OVERRIDES_RAW (string JSON or empty), EVENT_NAME, GITHUB_OUTPUT,
// BCS_KMS_CORE_TAG (set only for an RFC-021 pair with the pinned Blue).
// Unknown keys fail. Empty / {} means resolve as today.
// relayer_sdk_version is empty unless set, so only @fhevm/sdk runs.
// BCS_KMS_CORE_TAG replaces the kms_repo_ref / kms_core_version defaults, so
// keys come from Blue's kms release; explicit overrides still win.

const ALLOWED = new Set([
  "solana_programs_version",
  "host_contracts_version",
  "gateway_contracts_version",
  "contracts_chart_version",
  "coprocessor_version",
  "coprocessor_chart_version",
  "coprocessor_infra_chart_version",
  "kms_connector_version",
  "kms_connector_chart_version",
  "kms_core_version",
  "kms_repo_ref",
  "relayer_version",
  "relayer_http_version",
  "relayer_sdk_version",
  "test_suite_version",
  "common_chart_version",
  "redis_chart_version",
  "listener_chart_version",
  "listener_version",
]);

const ALWAYS_DEFAULTS = {
  coprocessor_infra_chart_version: "0.6.5",
  common_chart_version: "0.3.3",
  // zama-ai/kms v0.15.0-1. Image tag is github.ref_name on the kms release
  // docker-build; keep both keys on the same release.
  kms_repo_ref: "v0.15.0-1",
  kms_core_version: "v0.15.0-1",
  redis_chart_version: "25.3.8",
};

const bcsKmsCoreTag = String(process.env.BCS_KMS_CORE_TAG || "").trim();
if (bcsKmsCoreTag) {
  ALWAYS_DEFAULTS.kms_repo_ref = bcsKmsCoreTag;
  ALWAYS_DEFAULTS.kms_core_version = bcsKmsCoreTag;
}

const fs = require("node:fs");

const raw = String(process.env.OVERRIDES_RAW || "").trim();
let parsed = {};
if (raw && raw !== "{}") {
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    console.error(`::error::overrides is not valid JSON: ${error.message}`);
    process.exit(1);
  }
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    console.error("::error::overrides must be a JSON object");
    process.exit(1);
  }
  const unknown = Object.keys(parsed).filter((key) => !ALLOWED.has(key));
  if (unknown.length > 0) {
    console.error(`::error::unknown overrides keys: ${unknown.join(", ")}`);
    process.exit(1);
  }
  for (const [key, value] of Object.entries(parsed)) {
    if (value !== undefined && value !== null && typeof value !== "string") {
      console.error(`::error::overrides.${key} must be a string`);
      process.exit(1);
    }
  }
}

const out = {};
for (const key of ALLOWED) {
  const hasKey = Object.prototype.hasOwnProperty.call(parsed, key);
  const value = hasKey ? String(parsed[key] ?? "").trim() : "";
  if (value) {
    out[key] = value;
  } else if (ALWAYS_DEFAULTS[key]) {
    // Empty or omitted → workflow pin (same as `inputs.x || 'pin'`).
    out[key] = ALWAYS_DEFAULTS[key];
  } else {
    out[key] = "";
  }
}

const dest = process.env.GITHUB_OUTPUT;
if (!dest) {
  console.error("::error::GITHUB_OUTPUT is not set");
  process.exit(1);
}
const lines = [`overrides_json=${JSON.stringify(out)}`, ...Object.entries(out).map(([k, v]) => `${k}=${v}`)];
fs.appendFileSync(dest, `${lines.join("\n")}\n`);
console.log(`parsed overrides: ${JSON.stringify(out)}`);
