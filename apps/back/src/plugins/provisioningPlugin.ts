/**
 * Interface for ProvisioningPlugin
 * @interface ProvisioningPlugin
 */
export abstract class ProvisioningPlugin {
  /**
   * Unique slug to identify plugin such as "kong-konnect" or "auth0-jwt"
   * @type {string}
   */
  slug!: string;

  /**
   * Get a user from the gateway or auth provider
   * @param {string} customerId - The ID of the customer.
   * @param {string} email - The email of the user.
   * @returns {any} - The normalized user object.
   */
  getUser(customerId: string, email: string): any {
    throw new Error("Method not implemented.");
  }

  /**
   * Create a new user
   * @param {string} customerId - The ID of the customer.
   * @param {string} email - The email of the user.
   * @param {string} subscriptionId - The billing subscription ID.
   * @returns {any} - The normalized user object.
   */
  provisionUser(
    customerId: string,
    email: string,
    subscriptionId: string
  ): any {
    throw new Error("Method not implemented.");
  }

  /**
   * Create a new API Key
   * @param {string} customerId - The ID of the customer.
   * @param {string} email - The email of the user.
   * @returns {string} - The API key.
   */
  createApiKey(customerId: string, email: string): Promise<string> {
    throw new Error("Method not implemented.");
  }

  /**
   * Read config from envvars. Backwards Compatible with old envvar names
   * @param {string} name - Name of the envvar
   * @returns {string | undefined} - The envvar value.
   */
  getConfig(name: string): string | undefined {
    return process.env[name] || process.env[name.replace("PLUGIN_", "")];
  }
}
