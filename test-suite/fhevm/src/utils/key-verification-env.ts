/**
 * KMSGeneration lives on the canonical (first) host chain only. When a test run is
 * retargeted at another host chain, the per-chain overrides replace the canonical
 * RPC URL, chain id and contract addresses in the test container, so the SDK would
 * read KMSGeneration's address on the wrong chain. These variables carry the
 * canonical chain's values under their own names; `test/instance.ts` in the e2e
 * suite uses them to verify the FHE key / CRS against the canonical host.
 *
 * Returns nothing for the canonical chain (index 0): it reads KMSGeneration directly.
 */
export const keyVerificationEnv = (
  testSuiteEnv: Record<string, string | undefined>,
  index: number,
): Record<string, string> => {
  if (index === 0) {
    return {};
  }
  const required = (key: string): string => {
    const value = testSuiteEnv[key];
    if (!value) {
      throw new Error(`key verification on a non-canonical host chain requires ${key} in the test-suite environment`);
    }
    return value;
  };
  return {
    KEY_VERIFICATION_RPC_URL: required("RPC_URL"),
    KEY_VERIFICATION_CHAIN_ID: required("CHAIN_ID_HOST"),
    KEY_VERIFICATION_ACL_CONTRACT_ADDRESS: required("ACL_CONTRACT_ADDRESS"),
    KEY_VERIFICATION_KMS_VERIFIER_CONTRACT_ADDRESS: required("KMS_VERIFIER_CONTRACT_ADDRESS"),
    KEY_VERIFICATION_INPUT_VERIFIER_CONTRACT_ADDRESS: required("INPUT_VERIFIER_CONTRACT_ADDRESS"),
    KEY_VERIFICATION_PROTOCOL_CONFIG_CONTRACT_ADDRESS: required("PROTOCOL_CONFIG_CONTRACT_ADDRESS"),
  };
};
