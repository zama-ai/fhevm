import type { PackageName } from "./config";

export const ABI_COMPAT_EXCEPTIONS: Partial<Record<PackageName, Partial<Record<string, string[]>>>> = {
  "host-contracts": {
    ACL: ["error ExpirationDateBeforeOneHour()"],
    KMSVerifier: ["event NewContextSet(address[],uint256)"],
  },
  "gateway-contracts": {
    Decryption: [
      "error AccountNotAllowedToUseCiphertext(bytes32,address)",
      "error PublicDecryptNotAllowed(bytes32)",
      "error UserDecryptionNotDelegated(uint256,address,address,address)",
      "function isDelegatedUserDecryptionReady((address,address),(bytes32,address)[],bytes) returns (bool)",
    ],
    GatewayConfig: [
      "event InitializeGatewayConfig((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])",
      "event UpdateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)",
      "function getPublicDecryptionThreshold() returns (uint256)",
      "function getUserDecryptionThreshold() returns (uint256)",
    ],
  },
};
