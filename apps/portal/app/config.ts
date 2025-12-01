"use client";

const isBrowser = typeof window !== "undefined";
type ConfigKey = keyof NonNullable<Window["VITE_CONFIG"]>;
function getEnvVar(key: ConfigKey): string | undefined {
  if (isBrowser) {
    return window.VITE_CONFIG?.[key] ?? import.meta.env[key];
  }
  return import.meta.env[key];
}

const config = {
  stripe: {
    publishableKey: getEnvVar("VITE_STRIPE_PUBLISHABLE_KEY"),
    managementUrl: getEnvVar("VITE_STRIPE_MANAGEMENT_URL"),
  },
};

export { config };
