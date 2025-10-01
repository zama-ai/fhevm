import { ProvisioningPlugin } from "../plugins/provisioningPlugin";

/**
 * Returns an instance of the API Management provisioning plugin based on environment variables.
 * Reads both PLUGIN_APIM_PROVIDER and APIM_PROVIDER for backwards compatibility.
 */
export function getApimProvisioningPlugin(): ProvisioningPlugin | undefined {
  // Read both envvars for backwards compatibility
  const apimProvider =
    process.env.PLUGIN_APIM_PROVIDER || process.env.APIM_PROVIDER;
  if (!apimProvider) {
    console.error(
      "No PLUGIN_APIM_PROVIDER found. Please create an .env file with PLUGIN_APIM_PROVIDER one of the supported API management providers or edit the code to connect to your API Management."
    );
    return;
  }
  switch (apimProvider.toLowerCase()) {
    case "kong": {
      // Use dynamic import to avoid loading unless needed
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      const {
        KongProvisioningPlugin,
      } = require("../plugins/kong-gateway/kongProvisioningPlugin");
      return new KongProvisioningPlugin();
    }
    default:
      console.log(`Invalid apimProvider ${apimProvider} defined.`);
      return;
  }
}
