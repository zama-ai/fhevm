import type { Signature } from "@solana/kit";

import { demoApiFetch } from "./demoAuthorization";
import { encodeOperatorRequest, type OperatorRequest } from "./demoApi";

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
