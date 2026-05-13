import type { PackageName } from "./config";

export const ABI_COMPAT_EXCEPTIONS: Partial<Record<PackageName, Partial<Record<string, string[]>>>> = {
  "host-contracts": {
    ACL: ["error ExpirationDateBeforeOneHour()"],
    // Note: initializeFromEmptyProxy signature change (4-param → 2-param) is excluded
    // by the /^initialize/ regex in config.ts and does not need an explicit exception.
    // FHEVMExecutor declarations whose only revert sites were removed as unreachable
    // dead code (see issue #1370). No deployed call path can emit these selectors.
    FHEVMExecutor: ["error InvalidByteLength(uint8,uint256)", "error SecondOperandIsNotScalar()"],
    KMSVerifier: [
      "event NewContextSet(address[],uint256)",
      "function defineNewContext(address[],uint256)",
      "function destroyKmsContext(uint256)",
      "function reinitializeV2()",
      "event NewContextSet(uint256,address[],uint256)",
      "event KMSContextDestroyed(uint256)",
      "error KMSAlreadySigner()",
      "error KMSSignerNull()",
      "error SignersSetIsEmpty()",
      "error ThresholdIsNull()",
      "error ThresholdIsAboveNumberOfSigners()",
      "error InvalidKMSContext(uint256)",
      "error CurrentKMSContextCannotBeDestroyed(uint256)",
    ],
  },
  "gateway-contracts": {
    // The NotCustodian{Signer,TxSender} selectors were inherited from the GatewayConfigChecks
    // base by every concrete consumer below. The base helper that emitted them was removed as
    // unreachable dead code in issue #1370, so they disappear from each derived ABI.
    CiphertextCommits: ["error NotCustodianSigner(address)", "error NotCustodianTxSender(address)"],
    Decryption: [
      "error AccountNotAllowedToUseCiphertext(bytes32,address)",
      "error PublicDecryptNotAllowed(bytes32)",
      "error UserDecryptionNotDelegated(uint256,address,address,address)",
      "function isDelegatedUserDecryptionReady((address,address),(bytes32,address)[],bytes) returns (bool)",
      "error NotCustodianSigner(address)",
      "error NotCustodianTxSender(address)",
    ],
    InputVerification: ["error NotCustodianSigner(address)", "error NotCustodianTxSender(address)"],
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
