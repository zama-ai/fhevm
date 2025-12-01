import { useEffect, useMemo, useState } from "react";
import {
  Form,
  useActionData,
  useNavigation,
  type ActionFunctionArgs,
  type LoaderFunctionArgs,
} from "react-router";
import { Button } from "~/components/ui/button";
import { Copy, Loader2, Trash2 } from "lucide-react";
import {
  createApiKeyForUser,
  deleteApiKeyForUser,
  listApiKeysForUser,
  type ApiKey,
} from "~/features/api-keys/kong.server";
import { requireSubscription } from "~/lib/logic.server";
import type { Route } from "./+types/api-keys";

type ActionData =
  | {
      success: true;
      createdKey?: string;
      deletedKeyId?: string;
      message?: string;
    }
  | {
      success: false;
      error: string;
    }
  | undefined;

function maskKey(value?: string | null) {
  if (!value) return "••••";
  if (value.length <= 8) return `••••${value.slice(-4)}`;
  return `•••• ${value.slice(-4)}`;
}

function formatTimestamp(timestamp?: number | null) {
  if (!timestamp) return "—";
  const millis = timestamp > 1_000_000_000_000 ? timestamp : timestamp * 1000;
  const date = new Date(millis);
  if (Number.isNaN(date.getTime())) {
    return "—";
  }
  return date.toLocaleString();
}

export async function loader({ request }: LoaderFunctionArgs) {
  const { customer } = await requireSubscription(request);

  try {
    const apiKeys = await listApiKeysForUser(customer.email!);
    apiKeys.sort((a, b) => (b.created_at ?? 0) - (a.created_at ?? 0));
    return { apiKeys };
  } catch (error) {
    console.error("Failed to load API keys", error);
    return {
      apiKeys: [],
      error: "Unable to load API keys right now. Please try again later.",
    };
  }
}

export async function action({
  request,
}: ActionFunctionArgs): Promise<ActionData> {
  const { customer } = await requireSubscription(request);
  const formData = await request.formData();
  const intent = formData.get("intent");

  try {
    if (intent === "create") {
      const createdKey = await createApiKeyForUser(customer.email!);
      if (!createdKey?.key) {
        throw new Error("Kong did not return the created key value");
      }

      return {
        success: true,
        createdKey: createdKey.key,
        message: "New API key generated",
      };
    }

    if (intent === "delete") {
      const apiKeyId = formData.get("apiKeyId");
      if (typeof apiKeyId !== "string" || !apiKeyId) {
        return {
          success: false,
          error: "Missing API key identifier",
        };
      }

      await deleteApiKeyForUser({
        email: customer.email!,
        apiKeyId,
      });

      return {
        success: true,
        deletedKeyId: apiKeyId,
        message: "API key deleted",
      };
    }

    return {
      success: false,
      error: "Unsupported action",
    };
  } catch (error) {
    console.error("API key action failed", error);
    return {
      success: false,
      error:
        error instanceof Error
          ? error.message
          : "Unable to complete the request.",
    };
  }
}

export default function ApiKeysRoute({ loaderData }: Route.ComponentProps) {
  const { apiKeys, error } = loaderData;
  const actionData = useActionData<ActionData>();
  const navigation = useNavigation();
  const [copyState, setCopyState] = useState<"idle" | "copied" | "error">(
    "idle"
  );

  const isCreating =
    navigation.state === "submitting" &&
    navigation.formData?.get("intent") === "create";

  const deletingKeyId = useMemo(() => {
    if (navigation.state !== "submitting") return null;
    if (navigation.formData?.get("intent") !== "delete") return null;
    const apiKeyId = navigation.formData.get("apiKeyId");
    return typeof apiKeyId === "string" ? apiKeyId : null;
  }, [navigation]);

  useEffect(() => {
    if (actionData?.success && actionData.createdKey) {
      setCopyState("idle");
    }
  }, [actionData]);

  async function handleCopy(key: string) {
    if (typeof navigator === "undefined" || !navigator.clipboard) {
      setCopyState("error");
      return;
    }

    try {
      await navigator.clipboard.writeText(key);
      setCopyState("copied");
      setTimeout(() => setCopyState("idle"), 2000);
    } catch (copyError) {
      console.error("Failed to copy API key", copyError);
      setCopyState("error");
    }
  }

  return (
    <div className="container mx-auto max-w-4xl py-16">
      <div className="mb-10 space-y-2">
        <h1 className="text-3xl font-semibold">API Keys</h1>
        <p className="text-muted-foreground">
          Generate and manage API keys for accessing the Zama Console APIs. Keep
          your keys secure and rotate them regularly.
        </p>
      </div>

      <div className="mb-8">
        <Form method="post">
          <input type="hidden" name="intent" value="create" />
          <Button type="submit" disabled={isCreating}>
            {isCreating && (
              <Loader2 className="size-4 animate-spin" aria-hidden="true" />
            )}
            Generate new API key
          </Button>
        </Form>
      </div>

      {actionData?.success && actionData.createdKey && (
        <div className="mb-8 rounded-lg border border-primary/30 bg-primary/5 p-4">
          <h2 className="text-lg font-semibold">New API key</h2>
          <p className="mt-2 text-sm text-muted-foreground">
            Copy this key and store it securely. You will not be able to view it
            again.
          </p>
          <div className="mt-4 flex flex-col gap-3 sm:flex-row sm:items-center">
            <code className="grow overflow-hidden text-ellipsis rounded-md bg-background px-3 py-2 text-sm">
              {actionData.createdKey}
            </code>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleCopy(actionData.createdKey!)}
            >
              <Copy className="size-4" aria-hidden="true" />
              {copyState === "copied"
                ? "Copied"
                : copyState === "error"
                ? "Copy failed"
                : "Copy to clipboard"}
            </Button>
          </div>
        </div>
      )}

      {actionData && !actionData.success && actionData.error && (
        <div className="mb-6 rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive">
          {actionData.error}
        </div>
      )}

      {error && (
        <div className="mb-6 rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive">
          {error}
        </div>
      )}

      <div className="rounded-xl border bg-card shadow-sm">
        {apiKeys.length === 0 ? (
          <div className="p-8 text-center text-muted-foreground">
            No API keys yet. Generate one to get started.
          </div>
        ) : (
          <ul className="divide-y">
            {apiKeys.map((apiKey: ApiKey) => (
              <li
                key={apiKey.id}
                className="flex flex-col gap-4 px-6 py-5 sm:flex-row sm:items-center sm:justify-between"
              >
                <div>
                  <p className="font-medium">{maskKey(apiKey.key)}</p>
                  <p className="text-sm text-muted-foreground">
                    Created {formatTimestamp(apiKey.created_at)}
                  </p>
                </div>

                <Form method="post" className="flex items-center gap-3">
                  <input type="hidden" name="intent" value="delete" />
                  <input type="hidden" name="apiKeyId" value={apiKey.id} />
                  <Button
                    type="submit"
                    variant="destructive"
                    size="sm"
                    disabled={deletingKeyId === apiKey.id}
                  >
                    {deletingKeyId === apiKey.id ? (
                      <Loader2
                        className="size-4 animate-spin"
                        aria-hidden="true"
                      />
                    ) : (
                      <Trash2 className="size-4" aria-hidden="true" />
                    )}
                    Delete
                  </Button>
                </Form>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
