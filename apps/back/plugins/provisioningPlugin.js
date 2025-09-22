/**
 * Interface for ProvisioningPlugin
 * @interface ProvisioningPlugin
 */
class ProvisioningPlugin {
  
    /**
     * Unique slug to identify plugin such as "kong-konnect" or "auth0-jwt"
     * @type {string}
     */
    slug;
  
    /**
     * Get a user from the gateway or auth provider
     * @param {string} customerId - The ID of the customer.
     * @param {string} email - The email of the user.
     * @returns {string} - The normalized user object.
     */
    getUser(customerId, email) {
      throw new Error('Method not implemented.');
    }
    
    /**
     * Create a new user
     * @param {string} customerId - The ID of the customer.
     * @param {string} email - The email of the user.
     * @param {string} subscriptionId - The billing subscription ID.
     * @returns {string} - The normalized user object.
     */
    provisionUser(customerId, email, subscriptionId)  {
      throw new Error('Method not implemented.');
    }
    
    /**
     * Create a new API Key
     * @param {string} customerId - The ID of the customer.
     * @param {string} email - The email of the user.
     * @param {string} userId - The ID of the user.
     * @returns {string} - The API key.
     */
    createApiKey(customerId, email) {
      throw new Error('Method not implemented.');
    }

      /**
     * Read config from envvars. Backwards Compatible with old envvar names
     * @param {string} name - Name of the envvar
     * @returns {string} - The envvar value.
     */
    getConfig(name) {
      return process.env[name] || process.env[name.replace('PLUGIN_', '')]
    }
  }
  
  module.exports = {
    ProvisioningPlugin,
  };
  