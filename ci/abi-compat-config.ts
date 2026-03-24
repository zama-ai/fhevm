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

export const EXCLUDED_FUNCTION_PATTERNS = [/^initialize/, /^reinitializeV\d+$/];

export const PACKAGE_CONFIG: Record<
  PackageName,
  {
    contracts: string[];
    extraDeps?: string;
  }
> = {
  "host-contracts": {
    contracts: ["ACL", "FHEVMExecutor", "InputVerifier", "KMSVerifier"],
    extraDeps: "forge soldeer install",
  },
  "gateway-contracts": {
    contracts: ["Decryption", "GatewayConfig", "InputVerification", "KMSGeneration"],
  },
};
