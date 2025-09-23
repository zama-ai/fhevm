const config = {
  devPortalApiServer:
    window.VITE_CONFIG?.REACT_APP_DEV_PORTAL_API_SERVER ||
    import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER,
  auth0: {
    domain:
      window.VITE_CONFIG?.REACT_APP_AUTH0_DOMAIN ||
      import.meta.env.REACT_APP_AUTH0_DOMAIN,
    clientId:
      window.VITE_CONFIG?.REACT_APP_AUTH0_CLIENT_ID ||
      import.meta.env.REACT_APP_AUTH0_CLIENT_ID,
  },
  moesif: {
    publishableApplicationId:
      window.VITE_CONFIG?.REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID ||
      import.meta.env.REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID,
  },
  paymentProvider: "stripe",
  stripe: {
    publishableKey:
      window.VITE_CONFIG?.REACT_APP_STRIPE_PUBLISHABLE_KEY ||
      import.meta.env.REACT_APP_STRIPE_PUBLISHABLE_KEY,
    managementUrl:
      window.VITE_CONFIG?.REACT_APP_STRIPE_MANAGEMENT_URL ||
      import.meta.env.REACT_APP_STRIPE_MANAGEMENT_URL,
  },
};

export default config;
