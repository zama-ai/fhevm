import { Moesif } from "moesif-browser-js";

declare global {
  interface Window {
    moesif: Moesif;
    VITE_CONFIG?: {
      REACT_APP_DEV_PORTAL_API_SERVER?: string;
      REACT_APP_AUTH0_DOMAIN?: string;
      REACT_APP_AUTH0_CLIENT_ID?: string;
      REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID?: string;
      REACT_APP_STRIPE_PUBLISHABLE_KEY?: string;
      REACT_APP_STRIPE_MANAGEMENT_URL?: string;
    };
  }
}
