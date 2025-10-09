import { useQuery } from "@tanstack/react-query";
import useAuth from "./useAuth";
import { ApiKey } from "../types/api-key";
import config from "../config";

async function fetchApiKeys(idToken: string): Promise<ApiKey[]> {
  const res = await fetch(`${config.devPortalApiServer}/api-keys`, {
    headers: {
      Accept: "application/json",
      Authorization: `Bearer ${idToken}`,
    },
  });
  if (!res.ok) {
    console.error(`Failed to fetch API Keys`);
    throw new Error("Failed to fetch API Keys");
  }
  const apiKeys = (await res.json()) as ApiKey[];
  return apiKeys;
}

export function useApiKeys() {
  const { idToken } = useAuth();

  const {
    data: apiKeys,
    isLoading,
    error,
  } = useQuery({
    queryKey: ["api-keys", idToken],
    queryFn: () => fetchApiKeys(idToken!),
    enabled: !!idToken,
  });

  return { apiKeys, isLoading, error };
}
