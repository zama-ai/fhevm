// Load the installed SDK and demo only when a live scenario needs them.
// Offline orchestration tests inject their dependencies.

type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
export const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

type SdkVerifyModule = typeof import("@fhevm/sdk/solana");
let sdkVerifyModulePromise: Promise<SdkVerifyModule> | undefined;
export const sdkVerifyModule = (): Promise<SdkVerifyModule> =>
  (sdkVerifyModulePromise ??= import("@fhevm/sdk/solana"));
