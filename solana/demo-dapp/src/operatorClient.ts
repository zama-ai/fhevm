import type { DepositResult } from "./deposit";
import type { VaultDirection } from "./vaultRoots";

export const runDemoOperatorAction = async (
  action: "dispatch" | "settle",
  deposit: DepositResult,
  direction: VaultDirection = "deposit",
): Promise<void> => {
  const response = await fetch("/api/demo-operator", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      action,
      direction,
      batchIndex: deposit.batchIndex.toString(),
      batch: deposit.batch,
      amountBaseUnits: deposit.amountBaseUnits.toString(),
    }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo operator failed with HTTP ${response.status}`);
  }
};
