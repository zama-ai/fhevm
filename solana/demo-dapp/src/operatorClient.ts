import type { BatchPosition, OperatorAction, VaultDirection } from "./batchTypes";
import { demoApiFetch } from "./demoAuthorization";
import { encodeOperatorRequest } from "./demoApi";

export const runDemoOperatorAction = async (
  action: OperatorAction,
  deposit: BatchPosition,
  direction: VaultDirection = "deposit",
): Promise<void> => {
  const response = await demoApiFetch("/api/demo-operator", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(encodeOperatorRequest(action, direction, deposit)),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo operator failed with HTTP ${response.status}`);
  }
};
