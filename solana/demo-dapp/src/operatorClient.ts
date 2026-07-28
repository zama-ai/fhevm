import type { BatchTarget } from "./batchTypes";
import { demoApiFetch } from "./demoAuthorization";
import { encodeOperatorRequest, parseBatchTarget, type OperatorRequest } from "./demoApi";

export const prepareDemoDepositBatch = async (): Promise<BatchTarget> => {
  const response = await demoApiFetch("/api/demo-batch", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ direction: "deposit" }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo batch preparation failed with HTTP ${response.status}`);
  }
  return parseBatchTarget(await response.json());
};

export const runDemoOperatorAction = async (
  request: OperatorRequest,
): Promise<void> => {
  const response = await demoApiFetch("/api/demo-operator", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(encodeOperatorRequest(request)),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo operator failed with HTTP ${response.status}`);
  }
};
