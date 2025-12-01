export {};

declare global {
  interface Window {
    VITE_CONFIG?: {
      VITE_STRIPE_PUBLISHABLE_KEY?: string;
      VITE_STRIPE_MANAGEMENT_URL?: string;
    };
  }
}
