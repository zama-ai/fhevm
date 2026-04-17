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
    // KMSGeneration was frozen to a view-only implementation after the move of key generation to
    // Ethereum. State-changing entrypoints and the errors/events they emitted were removed from
    // the stable ABI surface; they are listed here to document the break.
    KMSGeneration: [
      "error CoprocessorSignerDoesNotMatchTxSender(address,address)",
      "error CrsgenNotRequested(uint256)",
      "error CrsgenOngoing(uint256)",
      "error ECDSAInvalidSignature()",
      "error ECDSAInvalidSignatureLength(uint256)",
      "error ECDSAInvalidSignatureS(bytes32)",
      "error EmptyKeyDigests(uint256)",
      "error HostChainNotRegistered(uint256)",
      "error KeygenNotRequested(uint256)",
      "error KeygenOngoing(uint256)",
      "error KmsAlreadySignedForCrsgen(uint256,address)",
      "error KmsAlreadySignedForKeygen(uint256,address)",
      "error KmsAlreadySignedForPrepKeygen(uint256,address)",
      "error KmsSignerDoesNotMatchTxSender(address,address)",
      "error NotCoprocessorSigner(address)",
      "error NotCoprocessorTxSender(address)",
      "error NotCustodianSigner(address)",
      "error NotCustodianTxSender(address)",
      "error NotKmsSigner(address)",
      "error NotKmsTxSender(address)",
      "error PrepKeygenNotRequested(uint256)",
      "event ActivateCrs(uint256,string[],bytes)",
      "event ActivateKey(uint256,string[],(uint8,bytes)[])",
      "event CrsgenRequest(uint256,uint256,uint8)",
      "event CrsgenResponse(uint256,bytes,bytes,address)",
      "event KeyReshareSameSet(uint256,uint256,uint256,uint8)",
      "event KeygenRequest(uint256,uint256)",
      "event KeygenResponse(uint256,(uint8,bytes)[],bytes,address)",
      "event PRSSInit()",
      "event PrepKeygenRequest(uint256,uint256,uint8)",
      "event PrepKeygenResponse(uint256,bytes,address)",
      "function getActiveCrsId() returns (uint256)",
      "function getActiveKeyId() returns (uint256)",
    ],
  },
};
