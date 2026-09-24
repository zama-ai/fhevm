#!/usr/bin/env bun
// CI-only receipt: begin before a cold up, finish only after that command succeeds.
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { STATE_FILE, REPO_ROOT } from "../src/layout";
import { readIdentityFile, receiptArtifacts, validateBuildReceipt } from "../src/consensus/build-provenance";
const [command, file, mode] = process.argv.slice(2);
if (!file) throw new Error("receipt path required");
const revision = () => {
  const result = Bun.spawnSync(["bash", "-c", 'source "$1/scripts/lib/source-revision.sh"; sr_revision "$2"', "receipt", `${REPO_ROOT}/test-suite/fhevm`, REPO_ROOT]);
  const sha = result.stdout.toString().trim();
  if (result.exitCode || !/^[a-f0-9]{40}$/.test(sha)) throw new Error("checkout build requires a clean source revision");
  return sha;
};
if (command === "begin") {
  if (existsSync(STATE_FILE)) throw new Error("CI checkout build requires a cold state; refusing to attribute cached state");
  if (!["checkout", "published"].includes(mode)) throw new Error("build mode required");
  // A checkout built with test failpoints is not the production feature set;
  // the receipt must say which one this run executed.
  const features = (process.env.FHEVM_CONSENSUS_TEST_FEATURES ?? "").trim().split(/\s+/).filter(Boolean).join(" ") || "none";
  writeFileSync(file, JSON.stringify({ revision: revision(), mode, features, startedAt: new Date().toISOString() }));
} else if (command === "finish") {
  const before = JSON.parse(readFileSync(file, "utf8"));
  const state = JSON.parse(readFileSync(STATE_FILE, "utf8"));
  if (before.revision !== revision() || !(Date.parse(state.updatedAt) >= Date.parse(before.startedAt))) throw new Error("source changed or build state is stale");
  const receipt = validateBuildReceipt({ ...before, completedAt: new Date().toISOString(), images: state.builtImages });
  writeFileSync(file, JSON.stringify(receipt));
} else if (command === "attach") {
  const receipt = validateBuildReceipt(JSON.parse(readFileSync(file, "utf8")));
  const identities = readIdentityFile(mode);
  for (const [key, value] of Object.entries(receiptArtifacts(receipt, identities))) console.log(`${key}=${value}`);
} else throw new Error("expected begin, finish or attach");
