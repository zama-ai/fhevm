import type { Signature } from "@solana/kit";

import type { BatchTarget, VaultDirection } from "./batchTypes";
import { demoApiFetch } from "./demoAuthorization";
import { encodeOperatorRequest, parseBatchTarget, type OperatorRequest } from "./demoApi";

export const prepareDemoBatch = async (direction: VaultDirection): Promise<BatchTarget> => {
  const response = await demoApiFetch("/api/demo-batch", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ direction }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo batch preparation failed with HTTP ${response.status}`);
  }
  return parseBatchTarget(await response.json());
};

export const prepareDemoDepositBatch = (): Promise<BatchTarget> => prepareDemoBatch("deposit");

export const runDemoOperatorAction = async (
  request: OperatorRequest,
): Promise<Signature | null> => {
  const response = await demoApiFetch("/api/demo-operator", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(encodeOperatorRequest(request)),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo operator failed with HTTP ${response.status}`);
  }
  const body = (await response.json()) as { readonly signature?: unknown };
  return typeof body.signature === "string" ? body.signature as Signature : null;
};
