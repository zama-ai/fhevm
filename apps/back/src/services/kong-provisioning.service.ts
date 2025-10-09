import { getLogger } from "../common/logger.context";
import { Span } from "../decorators/span";

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
 * KongProvisioningPlugin implements ProvisioningPlugin for Kong Gateway.
 */
export class KongProvisioningService {
  slug: string;
  kongUrl: string;
  kongAdminToken?: string;

  /**
   * Sanitize/validate the apiKeyId path segment to block SSRF via path traversal.
   * Only allows UUIDs or 24+/32+ hex, customize as appropriate to your backend.
   */
  private sanitizeApiKeyId(apiKeyId: string): string {
    // Example: only allow UUIDs or hex strings (Mongo IDs), adjust as necessary.
    // UUID v4 (with or without dashes): /^[0-9a-fA-F-]{36,}$/
    // Example relaxing to 24+-alphanumeric (Mongo, etc): /^[0-9a-zA-Z-]{24,64}$/
    // Tweak the regex to match only your valid ids.
    if (!/^[0-9a-zA-Z-]{24,64}$/.test(apiKeyId)) {
      throw new Error("Invalid API Key ID format");
    }
    return apiKeyId;
  }

  constructor() {
    this.slug = "kong-gateway";
    this.kongUrl = this.getConfig("PLUGIN_KONG_URL")?.trim() || "";
    this.kongAdminToken = this.getConfig("PLUGIN_KONG_ADMIN_TOKEN");
  }

  /**
   * Read config from envvars. Backwards Compatible with old envvar names
   * @param {string} name - Name of the envvar
   * @returns {string | undefined} - The envvar value.
   */
  private getConfig(name: string): string | undefined {
    return process.env[name] || process.env[name.replace("PLUGIN_", "")];
  }

  async kongAdminRequest<T = any>(
    method: "GET" | "DELETE",
    resource: string
  ): Promise<T>;
  async kongAdminRequest<T = any>(
    method: "POST" | "PUT" | "PATCH",
    resource: string,
    body: Record<string, unknown>
  ): Promise<T>;
  async kongAdminRequest<T = any>(
    method: string,
    resource: string,
    body?: Record<string, unknown>
  ): Promise<T> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "kongAdminRequest",
      httpMethod: method,
      resource,
    });
    const url = `${this.kongUrl}/${resource}`;
    // Redact sensitive key-auth IDs from logs.
    const redactedResource = resource.replace(
      /key-auth\/[^/]+/,
      "key-auth/[REDACTED]"
    );
    logger.debug(`${method} ${redactedResource}`);

    const response = await fetch(url, {
      method: method,
      headers: {
        ...(this.kongAdminToken
          ? { Authorization: `Bearer ${this.kongAdminToken}` }
          : undefined),
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    logger.debug(`response status code: ${response.status}`);
    if (!response.ok) {
      logger.error(`Failed: ${response.status}, ${response.statusText}`);
      throw new Error(
        `KongProvisioningPlugin Failed ${method} ${url}: ${response.status}, ${response.statusText}`
      );
    }

    if (response.status === 204) {
      logger.debug(`empty response`);
      return null as T;
    }
    return (await response.json()) as T;
  }

  @Span()
  async getUser(customerId: string, email: string): Promise<any> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "getUser",
      customerId,
      email,
    });
    logger.info("getUser");
    try {
      const resource = `consumers/${this.sanitizeEmail(email)}`;
      return await this.kongAdminRequest("GET", resource);
    } catch (error) {
      logger.error(error);
      throw new Error("KongProvisioningPlugin Failed to get user");
    }
  }

  @Span()
  async provisionUser(
    customerId: string,
    email: string,
    subscriptionId: string
  ): Promise<any> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "provisionUser",
      customerId,
      email,
      subscriptionId,
    });
    // create Consumer
    const kongConsumer = {
      username: this.sanitizeEmail(email),
      custom_id: customerId,
    };
    logger.debug(kongConsumer);
    const consumerResponse = await this.kongAdminRequest(
      "POST",
      `consumers/`,
      kongConsumer
    );
    return consumerResponse;
  }

  @Span()
  async createApiKey(customerId: string, email: string): Promise<ApiKey> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "createApiKey",
      customerId,
      email,
    });
    logger.debug(`creating API key`);
    // get kong consumer
    const resource = `consumers/${this.sanitizeEmail(email)}/key-auth`;
    const apiKey = await this.kongAdminRequest("POST", resource, {});
    logger.info(`KongProvisioningPlugin API Key created`);
    return apiKey;
  }

  @Span()
  async getApiKey(
    customerId: string,
    email: string,
    apiKeyId: string
  ): Promise<ApiKey> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "getApiKey",
      customerId,
      email,
      apiKeyId,
    });
    // Validate/sanitize apiKeyId
    const safeApiKeyId = this.sanitizeApiKeyId(apiKeyId);
    logger.debug(`listing API keys`);
    // get kong consumer
    const resource = `consumers/${this.sanitizeEmail(
      email
    )}/key-auth/${safeApiKeyId}`;
    const apiKey = await this.kongAdminRequest<ApiKey>("GET", resource);
    logger.info(`KongProvisioningPlugin API Key found`);
    return apiKey;
  }

  // (Add similar validation/sanitization in deleteApiKey if implemented here)
  @Span()
  async listApiKeys(customerId: string, email: string): Promise<ApiKey[]> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "listApiKeys",
      customerId,
      email,
    });
    logger.debug(`listing API keys`);
    // get kong consumer
    const resource = `consumers/${this.sanitizeEmail(email)}/key-auth`;
    const result = await this.kongAdminRequest<{ data: ApiKey[] }>(
      "GET",
      resource
    );
    logger.info(`KongProvisioningPlugin found ${result.data.length}`);
    return result.data;
  }

  @Span()
  async deleteApiKey(
    customerId: string,
    email: string,
    apiKeyId: string
  ): Promise<void> {
    const logger = getLogger().child({
      class: "KongProvisioningPlugin",
      method: "deleteApiKey",
      customerId,
      // email and apiKeyId are omitted from logger context to avoid logging sensitive information
    });
    // Do not log sensitive data such as email and apiKeyId
    logger.debug(`deleting API key`);
    const safeApiKeyId = this.sanitizeApiKeyId(apiKeyId);
    // get kong consumer
    const resource = `consumers/${this.sanitizeEmail(
      email
    )}/key-auth/${safeApiKeyId}`;
    await this.kongAdminRequest<void>("DELETE", resource);
    logger.info(`KongProvisioningPlugin API Key deleted`);
  }

  normalizeUser(user: any): any {
    // Normalize the user object as needed
    return {
      id: user.id,
      username: user.username,
      email: user.email,
      billing_customer_id: user.custom_id,
      billing_subscription_id: user.billing_subscription_id,
    };
  }

  sanitizeEmail(email: string): string {
    // Kong chokes up generating keys with percent encoded usernames. Do a simple version which removes + and % with a Kong safe char
    return encodeURIComponent(email).replaceAll("%", "__");
  }

  originalEmail(email: string): string {
    return decodeURIComponent(email.replaceAll("__", "%"));
  }
}

const provisioningService = new KongProvisioningService();
export default provisioningService;
