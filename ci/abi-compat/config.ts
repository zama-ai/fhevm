export type PackageName = "host-contracts" | "gateway-contracts";

export const EXCLUDED_MODIFIERS = new Set([
  "onlyOwner",
  "onlyACLOwner",
  "onlyGatewayOwner",
  "onlyFromEmptyProxy",
  "onlyPauser",
  "onlyCoprocessorTxSender",
  "onlyKmsTxSender",
  "onlyRegisteredHostChain",
  "onlyHandleFromRegisteredHostChain",
  "onlyDecryptionContract",
  "onlyInputVerificationContract",
]);

export const EXCLUDED_FUNCTION_PATTERNS = [
  /^initialize/,
  /^reinitializeV\d+$/,
  /^acceptOwnership$/,
  /^owner$/,
  /^transferOwnership$/,
  /^upgradeToAndCall$/,
];

// ABI coverage is derived from each package's upgrade-manifest.json.
// Keep only stable-surface exclusions here.
export const EXCLUDED_CONTRACT_FUNCTION_PATTERNS: Record<string, RegExp[]> = {
  HCULimit: [/^checkHCUFor/],
};

// Contracts that replace a different contract behind an existing proxy. While the baseline has no
// contract of the same name, the check compares the target against this predecessor. They stay out of
// upgrade-manifest.json because the upgrade tests deploy and upgrade each manifest entry by name.
// Keep this entry after the baseline release contains ProtocolConfigReplica.sol. This entry puts the
// contract in the check, and the check then compares against the baseline ProtocolConfigReplica.sol.
export const PREDECESSOR_CONTRACTS: Partial<Record<PackageName, Record<string, string>>> = {
  "host-contracts": { ProtocolConfigReplica: "ProtocolConfig" },
};

export const PACKAGE_CONFIG: Record<
  PackageName,
  {
    extraDeps?: string;
  }
> = {
  "host-contracts": {
    extraDeps: "forge soldeer install",
  },
  "gateway-contracts": {},
};
