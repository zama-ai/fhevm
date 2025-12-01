const KONG_URL = process.env.KONG_URL;
const KONG_ADMIN_TOKEN = process.env.KONG_ADMIN_TOKEN;

if (!KONG_URL) {
  throw new Error("Missing KONG_URL configuration in environment variables");
}

export interface ApiKey {
  created_at: number;
  id: string;
  key: string;
  consumer?: {
    id: string;
  };
  tags: string[] | null;
  ttl: number | null;
}

/**
 * Sanitize/validate the apiKeyId path segment to block SSRF via path traversal.
 * Only allows UUIDs or 24+/32+ hex, customize as appropriate to your backend.
 */
function sanitizeApiKeyId(apiKeyId: string): string {
  // Example: only allow UUIDs or hex strings (Mongo IDs), adjust as necessary.
  // UUID v4 (with or without dashes): /^[0-9a-fA-F-]{36,}$/
  // Example relaxing to 24+-alphanumeric (Mongo, etc): /^[0-9a-zA-Z-]{24,64}$/
  // Tweak the regex to match only your valid ids.
  if (!/^[0-9a-zA-Z-]{24,64}$/.test(apiKeyId)) {
    throw new Error("Invalid API Key ID format");
  }
  return apiKeyId;
}

function sanitizeEmail(email: string): string {
  // Kong chokes up generating keys with percent encoded usernames. Do a simple version which removes + and % with a Kong safe char
  return encodeURIComponent(email).replaceAll("%", "__");
}

function originalEmail(email: string): string {
  return decodeURIComponent(email.replaceAll("__", "%"));
}

// TODO: Fix user type
function normalizeUser(user: any): any {
  // Normalize the user object as needed
  return {
    id: user.id,
    username: user.username,
    email: user.email,
    billing_customer_id: user.custom_id,
    billing_subscription_id: user.billing_subscription_id,
  };
}

async function kongAdminRequest<T = any>(
  method: "GET" | "DELETE",
  resource: string
): Promise<T>;
async function kongAdminRequest<T = any>(
  method: "POST" | "PUT" | "PATCH",
  resource: string,
  body: Record<string, unknown>
): Promise<T>;
async function kongAdminRequest<T = any>(
  method: string,
  resource: string,
  body?: Record<string, unknown>
): Promise<T> {
  const url = `${KONG_URL}/${resource}`;
  // Redact sensitive key-auth IDs from logs.
  const redactedResource = resource.replace(
    /key-auth\/[^/]+/,
    "key-auth/[REDACTED]"
  );

  const response = await fetch(url, {
    method,
    headers: {
      ...(KONG_ADMIN_TOKEN
        ? { Authorization: `Bearer ${KONG_ADMIN_TOKEN}` }
        : {}),
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    ...(body ? { body: JSON.stringify(body) } : {}),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(
      `Kong Admin API request failed: ${response.status} ${response.statusText} - ${errorText}`
    );
  }

  const data = await response.json();
  return data;
}

export async function getUser(email: string) {
  try {
    // TODO: Fix user type
    // TODO: Handle error gracefully
    return await kongAdminRequest<any>(
      "GET",
      `consumers/${sanitizeEmail(email)}`
    );
  } catch (error) {
    console.error("Error fetching user from Kong:", error);
    throw new Error("Failed to fetch user from Kong");
  }
}

export async function provisionUser({
  customerId,
  email,
}: {
  customerId: string;
  email: string;
}) {
  return await kongAdminRequest<any>("POST", `consumers/`, {
    username: sanitizeEmail(email),
    custom_id: customerId,
  });
}

export async function createApiKeyForUser(email: string) {
  return await kongAdminRequest<any>(
    "POST",
    `consumers/${sanitizeEmail(email)}/key-auth`,
    {}
  );
}

export async function getApiKey({
  email,
  apiKeyId,
}: {
  email: string;
  apiKeyId: string;
}) {
  const safeApiKeyId = sanitizeApiKeyId(apiKeyId);
  const safeEmail = sanitizeEmail(email);
  return await kongAdminRequest<ApiKey>(
    "GET",
    `consumers/${safeEmail}/key-auth/${safeApiKeyId}`
  );
}

export async function listApiKeysForUser(email: string) {
  const safeEmail = sanitizeEmail(email);
  const response = await kongAdminRequest<{ data: ApiKey[] }>(
    "GET",
    `consumers/${safeEmail}/key-auth`
  );
  return response.data;
}

export async function deleteApiKeyForUser({
  email,
  apiKeyId,
}: {
  email: string;
  apiKeyId: string;
}) {
  const safeApiKeyId = sanitizeApiKeyId(apiKeyId);
  const safeEmail = sanitizeEmail(email);
  return await kongAdminRequest(
    "DELETE",
    `consumers/${safeEmail}/key-auth/${safeApiKeyId}`
  );
}
