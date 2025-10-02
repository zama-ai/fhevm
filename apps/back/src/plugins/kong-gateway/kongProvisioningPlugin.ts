import { ProvisioningPlugin } from "../provisioningPlugin";

/**
 * KongProvisioningPlugin implements ProvisioningPlugin for Kong Gateway.
 */
export class KongProvisioningPlugin extends ProvisioningPlugin {
  slug: string;
  kongUrl: string;
  kongAdminToken?: string;

  constructor() {
    super();
    this.slug = "kong-gateway";
    this.kongUrl = this.getConfig("PLUGIN_KONG_URL")?.trim() || "";
    this.kongAdminToken = this.getConfig("PLUGIN_KONG_ADMIN_TOKEN");
  }

  async kongAdminRequest(
    method: string,
    resource: string,
    body?: Record<string, unknown>
  ): Promise<any> {
    const url = `${this.kongUrl}/${resource}`;
    console.log(`KongProvisioningPlugin ${method} ${url}`);

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

    if (!response.ok) {
      console.warn(
        `KongProvisioningPlugin Failed ${method} ${url}: ${response.status}, ${response.statusText}`
      );
      throw new Error(
        `KongProvisioningPlugin Failed ${method} ${url}: ${response.status}, ${response.statusText}`
      );
    }

    return await response.json();
  }

  async getUser(customerId: string, email: string): Promise<any> {
    try {
      const resource = `consumers/${this.sanitizeEmail(email)}`;
      return await this.kongAdminRequest("GET", resource, undefined);
    } catch (error) {
      console.error(error);
      throw new Error("KongProvisioningPlugin Failed to get user");
    }
  }

  async provisionUser(
    customerId: string,
    email: string,
    subscriptionId: string
  ): Promise<any> {
    // create Consumer
    const kongConsumer = {
      username: this.sanitizeEmail(email),
      custom_id: customerId,
    };
    const consumerResponse = await this.kongAdminRequest(
      "POST",
      `consumers/`,
      kongConsumer
    );
    return consumerResponse;
  }

  async createApiKey(customerId: string, email: string): Promise<string> {
    // get kong consumer
    const keyAuthResource = `consumers/${this.sanitizeEmail(email)}/key-auth`;
    const keyAuthResult = await this.kongAdminRequest(
      "POST",
      keyAuthResource,
      {}
    );
    return keyAuthResult.key;
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
