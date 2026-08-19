// The vault module and the SDK sources are loaded lazily, not statically. Their tsconfig-path
// aliases resolve to real paths outside test-suite/fhevm, so their own dependencies (`@noble/*`,
// …) only resolve where the SDK's dependency graph is installed — clean-e2e.sh installs it with
// `npm ci --workspace=@fhevm/sdk…` before the up. The src/solana
// modules are also loaded by the OFFLINE `bun test src` run (through `two-holder-transfer.ts`,
// whose orchestration test injects fake dependencies), where that graph does not exist; the lazy
// seam keeps merely importing them dependency-free, the same reason `deposit-arc.scenario.test.ts`
// reaches the SDK through a dynamic import. The `typeof import(...)` types erase at runtime, so
// this module itself stays dependency-free too.

type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
export const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

type SdkProofModule = typeof import("@sdk-src/solana/proof.js");
let sdkProofModulePromise: Promise<SdkProofModule> | undefined;
export const sdkProofModule = (): Promise<SdkProofModule> =>
  (sdkProofModulePromise ??= import("@sdk-src/solana/proof.js"));

type SdkHandleModule = typeof import("@sdk-src/core/handle/FhevmHandle.js");
let sdkHandleModulePromise: Promise<SdkHandleModule> | undefined;
export const sdkHandleModule = (): Promise<SdkHandleModule> =>
  (sdkHandleModulePromise ??= import("@sdk-src/core/handle/FhevmHandle.js"));

type SdkVerifyModule = typeof import("@sdk-src/solana/actions/verifyPublicDecrypt.js");
let sdkVerifyModulePromise: Promise<SdkVerifyModule> | undefined;
export const sdkVerifyModule = (): Promise<SdkVerifyModule> =>
  (sdkVerifyModulePromise ??= import("@sdk-src/solana/actions/verifyPublicDecrypt.js"));
