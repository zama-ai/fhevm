export const migrationGpuRoles = ["tfhe-worker", "sns-worker", "zkproof-worker"] as const;
type Role = typeof migrationGpuRoles[number];
export type MigrationGpuReceipt = {
  mode?: "checkout" | "published";
  revision: string;
  releaseTag?: string;
  device: string;
  capability: string;
  images: Record<Role, string>;
  references?: Record<Role, string>;
};
type Image = { Id: string; Config: { Labels?: Record<string, string> | null } };
const digest = /^sha256:[0-9a-f]{64}$/;

export function validateMigrationGpuReceipt(receipt: MigrationGpuReceipt): void {
  if (![undefined, "checkout", "published"].includes(receipt.mode)) throw new Error("unknown GPU receipt mode");
  if (!/^[0-9a-f]{40}$/.test(receipt.revision)) throw new Error("GPU receipt requires a clean source revision");
  if (!/^GPU-[0-9a-f-]+$/.test(receipt.device) || !/^\d{2,3}$/.test(receipt.capability)) throw new Error("invalid GPU device class");
  if (!receipt.images || Object.keys(receipt.images).length !== migrationGpuRoles.length ||
      migrationGpuRoles.some(role => !digest.test(receipt.images[role] ?? ""))) throw new Error("GPU receipt requires all three immutable image IDs");
  if (receipt.mode !== "published") return;
  if (!/^v\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(receipt.releaseTag ?? "")) throw new Error("published GPU receipt requires an explicit release tag");
  for (const role of migrationGpuRoles) publishedCudaVersion(receipt, role);
}

function publishedCudaVersion(receipt: MigrationGpuReceipt, role: Role): string {
  const prefix = `ghcr.io/zama-ai/fhevm/coprocessor/${role}:${receipt.releaseTag}-cuda`;
  const reference = receipt.references?.[role] ?? "";
  if (!reference.startsWith(prefix)) throw new Error(`${role}: wrong published repository or release tag`);
  const suffix = /^(\d+\.\d+(?:\.\d+)?)-sm(\d+)@sha256:[0-9a-f]{64}$/.exec(reference.slice(prefix.length));
  if (!suffix || suffix[2] !== receipt.capability) throw new Error(`${role}: published reference must pin a digest and matching architecture`);
  return suffix[1]!;
}

/** Source equivalence permits test harness changes only. It never substitutes
 * the harness SHA for the published image's source label. */
export function validateMigrationGpuSource(receipt: MigrationGpuReceipt, checkout: string, releaseCommit?: string, changedPaths: string[] = []): void {
  if (!/^[0-9a-f]{40}$/.test(checkout)) throw new Error("migration requires a clean checkout");
  if (receipt.mode !== "published") {
    if (receipt.revision !== checkout) throw new Error("GPU image source differs from checkout");
    return;
  }
  if (releaseCommit !== receipt.revision) throw new Error("release tag does not identify the receipt source");
  if (changedPaths.some(path => !path.startsWith("test-suite/"))) throw new Error("published GPU production source differs from checkout");
}

export function validateMigrationGpuImage(receipt: MigrationGpuReceipt, role: Role, image: Image): void {
  const labels = image.Config.Labels ?? {};
  if (image.Id !== receipt.images[role]) throw new Error(`${role}: immutable image identity mismatch`);
  if (labels["ai.zama.fhevm.gpu"] !== "true" || labels["ai.zama.fhevm.compute-capability"] !== receipt.capability) throw new Error(`${role}: GPU architecture labels mismatch`);
  const revision = labels["org.opencontainers.image.revision"];
  if (receipt.mode === "published") {
    if (revision !== undefined && revision !== receipt.revision) throw new Error(`${role}: conflicting image source revision`);
    if (labels["ai.zama.fhevm.cuda-version"] !== publishedCudaVersion(receipt, role)) throw new Error(`${role}: CUDA label disagrees with published reference`);
  } else if (revision !== receipt.revision) throw new Error(`${role}: missing or wrong checkout source revision`);
}
