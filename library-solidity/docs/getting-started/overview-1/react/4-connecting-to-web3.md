# Choosing the right wallet for your project

This section will guide you through the wallet setup process.

## Why Use Reown's Appkit?

- **Comprehensive Authentication**: Provides a seamless Web3 authentication experience with social logins.
- **Multi-Wallet Support**: Easily manage multiple wallets within your application.
- **Customizable Features**: Tailor the setup to your project's needs by enabling or disabling features.

For more details, refer to the [Appkit Documentation](https://docs.reown.com/appkit/overview).

## Configuring Appkit in `App.tsx`

Below is a sample configuration for setting up Appkit in your React application:

```typescript
const queryClient = new QueryClient();

createAppKit({
  adapters: [wagmiAdapter],
  defaultAccountTypes: { eip155: "eoa" },
  enableWalletGuide: false,
  projectId,
  networks,
  metadata,
  enableCoinbase: false,
  coinbasePreference: "smartWalletOnly",
  themeMode: "light" as const,
  themeVariables: {
    "--w3m-border-radius-master": "0",
    "--w3m-font-family": "Telegraf",
  },
  features: {
    legalCheckbox: true,
    analytics: true,
    swaps: false,
    send: false,
    history: false,
    connectMethodsOrder: ["email", "social", "wallet"],
  },
});
```

## Integrating a Different Wallet Provider

You can integrate any wallet provider of your choice. Here are some important considerations:

- **fhevmjs Compatibility**: Ensure specific configurations are met for `fhevmjs` like the

### Vite Configuration

Ensure your `vite.config.ts` is set up correctly to handle cross-origin policies:

```typescript
export default defineConfig({
  // other code
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});
```

This is important because it enables threads in the `fhevmjs`. However by setting this it might mess with some other 

- **Coinbase Setting**: `enableCoinbase: false` is required

### Why These Headers?

- **Cross-Origin-Opener-Policy**: Helps isolate your app from other origins, enhancing security.
- **Cross-Origin-Embedder-Policy**: Ensures resources are only loaded from trusted sources.

By following these guidelines, you'll be able to set up a robust and flexible wallet integration for your project. If you have any questions or need further customization, refer to the documentation or reach out to the community for support.