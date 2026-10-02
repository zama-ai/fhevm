#!/usr/bin/env bun
import { migrationGpuRoles, validateMigrationGpuImage, validateMigrationGpuReceipt, validateMigrationGpuSource, type MigrationGpuReceipt } from "../src/consensus/migration-gpu-provenance";
const [file, root, checkout, evidence] = process.argv.slice(2);
if (!file || !root || !checkout || !evidence) throw new Error("receipt, repository, checkout revision and evidence path required");
const receipt: MigrationGpuReceipt = await Bun.file(file).json();
validateMigrationGpuReceipt(receipt);
function command(args: string[]): string {
  const result = Bun.spawnSync(args, { stdout: "pipe", stderr: "pipe", timeout: 30_000 });
  if (result.exitCode !== 0) throw new Error(`${args[0]} failed during GPU provenance verification`);
  return result.stdout.toString().trim();
}
let releaseCommit: string | undefined;
let changedPaths: string[] = [];
if (receipt.mode === "published") {
  releaseCommit = command(["git", "-C", root, "rev-parse", `${receipt.releaseTag}^{commit}`]);
  changedPaths = command(["git", "-C", root, "diff", "--name-only", receipt.revision, checkout]).split("\n").filter(Boolean);
}
validateMigrationGpuSource(receipt, checkout, releaseCommit, changedPaths);
const observations = [];
for (const role of migrationGpuRoles) {
  const reference = receipt.mode === "published" ? receipt.references![role] : receipt.images[role];
  const images = JSON.parse(command(["docker", "image", "inspect", reference]));
  if (!Array.isArray(images) || images.length !== 1) throw new Error(`${role}: ambiguous image inspection`);
  validateMigrationGpuImage(receipt, role, images[0]);
  observations.push({role, reference, imageId: images[0].Id, sourceRevisionLabel: images[0].Config.Labels?.["org.opencontainers.image.revision"] ?? null});
}
await Bun.write(evidence, JSON.stringify({mode: receipt.mode ?? "checkout", checkout, sourceRevision: receipt.revision, releaseTag: receipt.releaseTag, changedPaths, observations,
  sourceAttestation: receipt.mode === "published" ? "Release tag and immutable registry artifacts; absent source labels are not an independent source attestation" : "Checkout-labelled local build receipt"}, null, 2) + "\n");
console.error(`GPU migration provenance verified (${receipt.mode ?? "checkout"}); evidence: ${evidence}`);
