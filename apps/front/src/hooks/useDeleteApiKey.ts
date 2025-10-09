import { useMutation, useQueryClient } from "@tanstack/react-query";
import useAuth from "./useAuth";
import config from "../config";

async function deleteApiKeyFn(
  idToken: string,
  apiKeyId: string
): Promise<void> {
  const res = await fetch(`${config.devPortalApiServer}/api-keys/${apiKeyId}`, {
    method: "DELETE",
    headers: {
      Accept: "application/json",
      Authorization: `Bearer ${idToken}`,
    },
  });
  if (!res.ok) {
    console.error(`Failed to delete API Key ${apiKeyId}`);
    throw new Error("Failed to delete API Key");
  }
}

export function useDeleteApiKey() {
  const { idToken } = useAuth();
  const queryClient = useQueryClient();

  const { mutate: deleteApiKey } = useMutation({
    mutationFn: (apiKeyId: string) => {
      if (!idToken) {
        return Promise.reject(new Error("idToken is not loaded"));
      }
      return deleteApiKeyFn(idToken, apiKeyId);
    },
    onSuccess: () => {
      if (idToken) {
        queryClient.invalidateQueries({ queryKey: ["api-keys", idToken] });
      }
    },
  });

  return { deleteApiKey };
}
