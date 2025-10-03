import { ProvisioningPlugin } from "../plugins/provisioningPlugin";
import logger from "../common/logger";

/**
 * Returns an instance of the API Management provisioning plugin based on environment variables.
 * Reads both PLUGIN_APIM_PROVIDER and APIM_PROVIDER for backwards compatibility.
 */
export function getApimProvisioningPlugin(): ProvisioningPlugin | undefined {
  // Read both envvars for backwards compatibility
  const apimProvider =
    process.env.PLUGIN_APIM_PROVIDER || process.env.APIM_PROVIDER;
  if (!apimProvider) {
    logger.fatal(
      "No PLUGIN_APIM_PROVIDER found. Please create an .env file with PLUGIN_APIM_PROVIDER one of the supported API management providers or edit the code to connect to your API Management."
    );
    return;
  }
  switch (apimProvider.toLowerCase()) {
    case "kong": {
      logger.trace("Loading KongProvisioningPlugin");
      // Use dynamic import to avoid loading unless needed
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      const {
        KongProvisioningPlugin,
      } = require("../plugins/kong-gateway/kongProvisioningPlugin");
      return new KongProvisioningPlugin();
    }
    default:
      logger.error(`Invalid apimProvider ${apimProvider} defined.`);
      return;
  }
}
