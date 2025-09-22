function getApimProvisioningPlugin() {
  // Read both envvars for backwards compatibility
  const apimProvider =
    process.env.PLUGIN_APIM_PROVIDER || process.env.APIM_PROVIDER;
  if (!apimProvider) {
    console.error(
      "No PLUGIN_APIM_PROVIDER found. Please create an .env file with PLUGIN_APIM_PROVIDER one of the supported API management providers or edit the code to connect to your API Management."
    );
  }
  switch (apimProvider.toLowerCase()) {
    case "kong":
      const {
        KongProvisioningPlugin,
      } = require("../plugins/kong-gateway/kongProvisioningPlugin");
      return new KongProvisioningPlugin();
    default:
      console.log(`Invalid apimProvider ${apimProvider} defined.`);
  }
}

module.exports = {
  getApimProvisioningPlugin,
};
