// EXEMPLAR — the regenerate half of discover→regenerate (stack/EXEMPLAR.md §0 keystone).
//
// THE principle that collapses the old ~10k-LoC CLI: nothing downstream of a deploy may
// hardcode a contract address (or signer, or key id). Each deploy produces its actual
// addresses; a `discover` step reads them into Discovery; this module is the SINGLE place
// those discovered values are threaded into every consumer's config. One function, one map
// — instead of the same address copy-pasted across a dozen env files (which is exactly how
// the live cluster became an unbootable frankenstein: order-dependent addresses drift and
// the copies rot).
//
// `threadDiscovery` returns ConfigMap patches; the recipe's `regenerate` phase applies them
// (Stack.patchConfigMap) right before the consuming service starts. Adding a new consumer of
// a discovered value = one line here, nowhere else.

import type { Discovery, RecipeConfig } from "./recipe";

/** A ConfigMap key/value patch the regenerate step applies before dependent services start. */
export type ConfigPatch = { configMap: string; data: Record<string, string> };

const pubKeyUrl = (vault: string, id: string) => `${vault}/PUB/PublicKey/${id}`;
const serverKeyUrl = (vault: string, id: string) => `${vault}/PUB/ServerKey/${id}`;
const snsKeyUrl = (vault: string, id: string) => `${vault}/PUB/SnsKey/${id}`;
const crsUrl = (vault: string, id: string) => `${vault}/PUB/CRS/${id}`;

/**
 * threadDiscovery — the whole regenerate logic. Given the static config and what's been
 * discovered so far, produce the ConfigMap patches that thread discovered values into the
 * services. Only emits patches for values already discovered (so it is safe to call after
 * each discover step; undiscovered → no patch yet).
 */
export function threadDiscovery(cfg: RecipeConfig, disc: Discovery): ConfigPatch[] {
  const patches: ConfigPatch[] = [];
  const g = disc.gateway ?? {};
  const h = disc.host ?? {};
  const vault = cfg.publicVaultUrl;

  // kms-signer (discovered from kms-core) → both contract-deploy envs register it on-chain.
  if (disc.kmsSigner) {
    patches.push({ configMap: "host-sc-env", data: { KMS_SIGNER_ADDRESS_0: disc.kmsSigner } });
    patches.push({ configMap: "gateway-sc-env", data: { KMS_SIGNER_ADDRESS_0: disc.kmsSigner } });
  }

  // gateway contract addresses (order-dependent — MUST be discovered, never assumed) → every
  // service that talks to the gateway, and the mocked-payment approve step.
  if (g.inputVerification) {
    patches.push({ configMap: "coprocessor-env", data: { INPUT_VERIFICATION_ADDRESS: g.inputVerification } });
    patches.push({
      configMap: "relayer-env",
      data: { APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: g.inputVerification },
    });
    // host-sc-env: the host InputVerifier is deployed with verifyingContractSource = THIS gateway
    // InputVerification address (EIP712UpgradeableCrossChain). The coprocessor signs input
    // attestations under that cross-chain domain; if the host registers a STALE address the
    // domain mismatches and on-chain verification reverts InvalidSigner(<varying>) — every
    // transfer fails while plaintext mint passes. MUST be the discovered value (§0 finding 18).
    patches.push({ configMap: "host-sc-env", data: { INPUT_VERIFICATION_ADDRESS: g.inputVerification } });
  }
  if (g.decryption) {
    patches.push({ configMap: "coprocessor-env", data: { DECRYPTION_ADDRESS: g.decryption } });
    patches.push({
      configMap: "relayer-env",
      data: { APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: g.decryption },
    });
    // connector-env keys use the *_CONTRACT__ADDRESS form and require 0x-HEX (v0.13.0-6 rejects
    // anything not starting with 0x — the old manifest's decimal was wrong for this version).
    patches.push({ configMap: "connector-env", data: { KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: g.decryption } });
  }
  if (g.gatewayConfig)
    patches.push({ configMap: "connector-env", data: { KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: g.gatewayConfig } });
  if (g.protocolPayment && g.zamaOft) {
    // The mocked-payment approve must target the DISCOVERED ProtocolPayment (the live bug:
    // approving the template's stale 0x1ceF while the deploy put it at 0x3b12).
    patches.push({
      configMap: "gateway-mocked-payment-env",
      data: { PROTOCOL_PAYMENT_ADDRESS: g.protocolPayment, ZAMA_OFT_ADDRESS: g.zamaOft },
    });
  }

  // host contract addresses (deterministic, but still discovered to stay honest).
  if (h.acl) patches.push({ configMap: "coprocessor-env", data: { ACL_CONTRACT_ADDRESS: h.acl } });
  if (h.fhevmExecutor)
    patches.push({ configMap: "coprocessor-env", data: { FHEVM_EXECUTOR_CONTRACT_ADDRESS: h.fhevmExecutor } });
  if (h.kmsGeneration) {
    patches.push({ configMap: "coprocessor-env", data: { KMS_GENERATION_ADDRESS: h.kmsGeneration } });
    patches.push({
      configMap: "connector-env",
      data: { KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: h.kmsGeneration },
    });
  }
  if (h.kmsVerifier)
    patches.push({ configMap: "connector-env", data: { KMS_CONNECTOR_KMS_VERIFIER_ADDRESS: h.kmsVerifier } });

  // erc20 e2e (the L2 behavioral test, an in-cluster Job) reads every discovered host+gateway
  // address from erc20-env. Threading them here is what lets `up` drive the test SELF-CONTAINED —
  // no hand-set Job env, same discover→regenerate path as every other consumer.
  const erc20: Record<string, string> = {};
  if (h.acl) erc20.ACL_CONTRACT_ADDRESS = h.acl;
  if (h.fhevmExecutor) erc20.FHEVM_EXECUTOR_CONTRACT_ADDRESS = h.fhevmExecutor;
  if (h.kmsVerifier) erc20.KMS_VERIFIER_CONTRACT_ADDRESS = h.kmsVerifier;
  if (h.inputVerifier) erc20.INPUT_VERIFIER_CONTRACT_ADDRESS = h.inputVerifier;
  if (h.hcuLimit) erc20.HCU_LIMIT_CONTRACT_ADDRESS = h.hcuLimit;
  if (g.decryption) erc20.DECRYPTION_ADDRESS = g.decryption;
  if (g.inputVerification) erc20.INPUT_VERIFICATION_ADDRESS = g.inputVerification;
  if (Object.keys(erc20).length) patches.push({ configMap: "erc20-env", data: erc20 });

  // realized key id (from keygen) → coprocessor key material URLs + relayer keyurl.
  // The relayer keyurl is CONFIG-driven (not on-chain); this is the only feed of the active key.
  if (disc.fheKeyId) {
    patches.push({
      configMap: "coprocessor-env",
      data: {
        FHE_KEY_ID: disc.fheKeyId,
        KEY_ID: disc.fheKeyId,
        KMS_PUBLIC_KEY: pubKeyUrl(vault, disc.fheKeyId),
        KMS_SERVER_KEY: serverKeyUrl(vault, disc.fheKeyId),
        KMS_SNS_KEY: snsKeyUrl(vault, disc.fheKeyId),
      },
    });
    patches.push({ configMap: "relayer-env", data: { APP_KEYURL__FHE_PUBLIC_KEY__URL: pubKeyUrl(vault, disc.fheKeyId) } });
  }
  if (disc.crsId) {
    patches.push({ configMap: "coprocessor-env", data: { KMS_CRS_KEY: crsUrl(vault, disc.crsId) } });
    patches.push({ configMap: "relayer-env", data: { APP_KEYURL__CRS__URL: crsUrl(vault, disc.crsId) } });
  }

  return patches;
}
