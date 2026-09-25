import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { OVERRIDE_GROUPS } from "../types";

export interface BuildReceipt {
  revision: string;
  mode: "checkout" | "published";
  startedAt: string;
  completedAt: string;
  images: { ref: string; id: string; group: string }[];
}
const sha = /^sha256:[a-f0-9]{64}$/;
export function validateBuildReceipt(value: unknown): BuildReceipt {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("invalid checkout build receipt");
  const r = value as BuildReceipt;
  const timestamp = (v: unknown): v is string => typeof v === "string" &&
    /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z$/.test(v) && Number.isFinite(Date.parse(v));
  if (typeof r.revision !== "string" || !/^[a-f0-9]{40}$/.test(r.revision) ||
      typeof r.mode !== "string" || !["checkout", "published"].includes(r.mode) ||
      !timestamp(r.startedAt) || !timestamp(r.completedAt) || Date.parse(r.completedAt) < Date.parse(r.startedAt) || !Array.isArray(r.images) ||
      r.images.some((i) => !i || typeof i !== "object" || Array.isArray(i) || typeof i.ref !== "string" || !i.ref.trim() ||
        typeof i.group !== "string" || !(OVERRIDE_GROUPS as readonly string[]).includes(i.group) || typeof i.id !== "string" || !sha.test(i.id))) {
    throw new Error("invalid checkout build receipt");
  }
  for (const group of r.mode === "checkout" ? ["test-suite", "coprocessor"] : ["test-suite"]) {
    if (!r.images.some((i) => i.group === group)) throw new Error(`build receipt lacks ${group} images`);
  }
  if (new Set(r.images.map((i) => i.ref)).size !== r.images.length) throw new Error("duplicate build image reference");
  return r;
}
export function imageBindings(receipt: BuildReceipt, identities: Record<string, string>): void {
  if (!identities["image_fhevm-test-suite-e2e-debug"]) throw new Error("missing current E2E harness image observation");
  if (receipt.mode === "checkout" && !Object.keys(identities).some((name) => /^image_coprocessor\d*-tfhe-worker$/.test(name))) {
    throw new Error("missing current coprocessor worker image observation");
  }
  const seen = new Set<string>();
  const refs = new Set<string>();
  for (const [name, value] of Object.entries(identities)) {
    if (!name.startsWith("image_")) continue;
    const match = /^(sha256:[a-f0-9]{64})(?: .*?)? \(([^)]+)\)$/.exec(value);
    if (!match) throw new Error(`invalid immutable image observation: ${name}`);
    const built = receipt.images.find((image) => image.ref === match[2]);
    if (built) {
      if (built.id !== match[1]) throw new Error(`running image differs from checkout build: ${name}`);
      seen.add(built.group); refs.add(built.ref);
    } else if (name === "image_fhevm-test-suite-e2e-debug" ||
      (receipt.mode === "checkout" && /^image_coprocessor\d*-(tfhe-worker|sns-worker|zkproof-worker|host-listener(?:-poller|-consumer)?(?:-chain-[a-z0-9-]+)?|gw-listener|transaction-sender|consensus-detector|upgrade-controller)$/.test(name))) {
      throw new Error(`running repository image absent from checkout build: ${name}`);
    }
  }
  for (const built of receipt.images) {
    if ((built.group === "test-suite" || (receipt.mode === "checkout" && built.group === "coprocessor" && !/\/db-migration:/.test(built.ref))) && !refs.has(built.ref)) {
      throw new Error(`built repository runtime image has no current container observation: ${built.ref}`);
    }
  }
  for (const group of receipt.mode === "checkout" ? ["test-suite", "coprocessor"] : ["test-suite"]) {
    if (!seen.has(group)) throw new Error(`no observed ${group} image binds to checkout build`);
  }
}
export function receiptArtifacts(receipt: BuildReceipt, identities: Record<string, string>): Record<string, string> {
  validateBuildReceipt(receipt); imageBindings(receipt, identities);
  const body = JSON.stringify(receipt);
  return { checkout_build_receipt: body, checkout_build_receipt_sha256: createHash("sha256").update(body).digest("hex") };
}
export function verifyCheckoutArtifacts(identities: Record<string, string> | undefined, revision: string): void {
  if (!identities?.checkout_build_receipt) throw new Error("missing checkout build receipt");
  const receipt = validateBuildReceipt(JSON.parse(identities.checkout_build_receipt));
  if (receipt.mode !== "checkout" || receipt.revision !== revision) throw new Error("checkout build receipt has stale revision or published runtime");
  const expected = createHash("sha256").update(identities.checkout_build_receipt).digest("hex");
  if (expected !== identities.checkout_build_receipt_sha256) throw new Error("checkout build receipt digest mismatch");
  imageBindings(receipt, identities);
  if (identities.gpu_software_revision && identities.gpu_software_revision !== revision) throw new Error("GPU build revision differs from checkout");
}
export function parseIdentityText(text: string): Record<string, string> {
  const identities: Record<string, string> = {};
  for (const line of text.trim().split("\n")) {
    const separator = line.indexOf("=");
    if (separator < 1 || separator === line.length - 1) throw new Error("invalid run identity file");
    const key = line.slice(0, separator), value = line.slice(separator + 1);
    if (key in identities && identities[key] !== value) throw new Error(`conflicting run identity ${key}`);
    identities[key] = value;
  }
  return identities;
}

export const readIdentityFile = (file: string): Record<string, string> => parseIdentityText(readFileSync(file, "utf8"));
