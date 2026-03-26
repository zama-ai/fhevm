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
