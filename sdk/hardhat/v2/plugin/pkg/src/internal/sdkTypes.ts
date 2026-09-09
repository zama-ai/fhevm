import type { createFhevmCleartextClient } from '@fhevm/sdk/ethers/cleartext';

/**
 * Types that `@fhevm/sdk` defines but does not re-export from any public entry point, recovered by
 * inference so the plugin can still name them.
 *
 * This is a workaround, not a design: each entry here should become a plain
 * `import type { X } from "@fhevm/sdk/..."` once upstream exports it. See
 * `plans/HOST_CONTRACTS_CLEARTEXT_UPSTREAM_FIXES.md`.
 */

/**
 * The client both factories return.
 *
 * `@fhevm/sdk/types` exports `FhevmEncryptClient` and `FhevmDecryptClient` but not the combined
 * `FhevmClient` (it lives in `core/types/fhevmClient.d.ts`), so it is inferred from the cleartext
 * factory. `createFhevmClient` returns the same shape, which is what lets `FhevmExternalAPI` be
 * written once for local and public networks alike.
 */
export type FhevmClient = ReturnType<typeof createFhevmCleartextClient>;
