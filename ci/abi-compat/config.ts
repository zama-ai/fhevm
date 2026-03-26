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

export const EXCLUDED_CONTRACT_FUNCTION_PATTERNS: Record<string, RegExp[]> = {
  HCULimit: [/^checkHCUFor/],
};

export const PACKAGE_CONFIG: Record<
  PackageName,
  {
    contracts: string[];
    extraDeps?: string;
  }
> = {
  "host-contracts": {
    contracts: ["ACL", "FHEVMExecutor", "HCULimit", "InputVerifier", "KMSVerifier"],
    extraDeps: "forge soldeer install",
  },
  "gateway-contracts": {
    contracts: ["CiphertextCommits", "Decryption", "GatewayConfig", "InputVerification", "KMSGeneration"],
  },
};
