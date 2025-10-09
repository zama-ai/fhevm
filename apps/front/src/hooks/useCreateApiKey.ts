import { useMutation, useQueryClient } from "@tanstack/react-query";
import config from "../config";
import { ApiKey } from "../types/api-key";
import useAuth from "./useAuth";

async function createApiKeyFn(
  idToken: string,
  data: Partial<Pick<ApiKey, "tags" | "ttl">>
): Promise<ApiKey> {
  const res = await fetch(`${config.devPortalApiServer}/api-keys`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
      Authorization: `Bearer ${idToken}`,
    },
    body: JSON.stringify(data),
  });
  if (!res.ok) {
    console.error(`Failed to create API Key`);
    throw new Error("Failed to create API Key");
  }

  const apiKey = (await res.json()) as ApiKey;
  return apiKey;
}

export function useCreateApiKey() {
  const { idToken } = useAuth();
  const queryClient = useQueryClient();

  const { mutateAsync: createApiKey, error } = useMutation({
    mutationFn: (data: Partial<Pick<ApiKey, "tags" | "ttl">> = {}) =>
      createApiKeyFn(idToken!, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api-keys", idToken!] });
    },
  });

  return { createApiKey, error };
}
