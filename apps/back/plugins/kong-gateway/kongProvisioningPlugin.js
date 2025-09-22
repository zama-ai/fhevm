const { ProvisioningPlugin } = require("../provisioningPlugin");

class KongProvisioningPlugin extends ProvisioningPlugin {
    constructor() {
        super();
        this.slug = "kong-gateway";
        this.kongUrl = this.getConfig('PLUGIN_KONG_URL').trim()
        this.kongAdminToken = this.getConfig('PLUGIN_KONG_ADMIN_TOKEN')
    }

    async kongAdminRequest(method, resource, body) {
      const url = `${this.kongUrl}/${resource}`
      console.log(`KongProvisioningPlugin ${method} ${url}`)

      const response = await fetch(url,
        {
          method: method,
          headers: {
            Authorization: this.kongAdminToken ? `Bearer ${this.kongAdminToken}` : undefined,
            "Content-Type": "application/json",
            Accept: "application/json",
          },
          body: body ? JSON.stringify(body) : undefined
        }
      );
    
      if (!response.ok) {
        console.log(
          `KongProvisioningPlugin Failed ${method} ${url}: ${response.status}, ${response.statusText}`
        );
        throw new Error(`KongProvisioningPlugin Failed ${method} ${url}: ${response.status}, ${response.statusText}`);
      }
    
      return await response.json();
    }

    async getUser(customerId, email) {
        try {
          const consumerResource = `consumers/${this.sanitizeEmail(email)}`
          return await this.kongAdminRequest('GET', resource, undefined);
        } catch (error) {
            console.error(error);
            throw new Error('KongProvisioningPlugin Failed to get user');
        }
    }

    async provisionUser(customerId, email, subscriptionId) {
      // create Consumer
      const kongConsumer = {
        username: this.sanitizeEmail(email),
        custom_id: customerId,
      };
      const consumerResponse = await this.kongAdminRequest('POST', `consumers/`, kongConsumer);
      return consumerResponse;
    }

    async createApiKey(customerId, email) {    
      //get kong consumer    
      const keyAuthResource = `consumers/${this.sanitizeEmail(email)}/key-auth`
      const keyAuthResult = await this.kongAdminRequest('POST', keyAuthResource, {});
      return keyAuthResult.key;
    }

    normalizeUser(user) {
        // Normalize the user object as needed
        return {
            id: user.id,
            username: user.username,
            email: user.email,
            billing_customer_id: user.custom_id,
            billing_subscription_id: user.billing_subscription_id
        };
    }

    sanitizeEmail(email) {
      // Kong chokes up generating keys with percent encoded usernames. Do a simple version which removes + and % with a Kong safe char
      return encodeURIComponent(email).replaceAll('%', '__')
    }

    originalEmail(email) {
      return decodeURIComponent(email.replaceAll('__', '%'))
    }
}

module.exports = {
  KongProvisioningPlugin,
};