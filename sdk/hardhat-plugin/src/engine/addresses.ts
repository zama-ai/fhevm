import { Wallet } from "ethers";

/**
 * Canonical host-contract addresses. These are BAKED INTO the cleartext bytecode
 * (`host-contracts-cleartext/src/host-contracts/addresses/FHEVMHostAddresses.sol`), so the mock must
 * deploy each contract at exactly the address below or the contracts will not find one another.
 * Do not "configure" these — verify them (see deploy.ts `assertBakedInAddresses`).
 */
export const ADDRESSES = {
  ACL: "0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D",
  FHEVMExecutor: "0xe3a9105a3a932253A70F126eb1E3b589C643dD24",
  KMSVerifier: "0x901F8942346f7AB3a01F6D7613119Bca447Bb030",
  InputVerifier: "0x36772142b74871f255CbD7A3e89B401d3e45825f",
  HCULimit: "0x233ff88A48c172d29F675403e6A8e302b0F032D9",
  ProtocolConfig: "0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc",
  KMSGeneration: "0x216be43148dB537BeddBC268163deb1a802b5553",
  PauserSet: "0xded0D2a71268DC12622BdD1b55d68a1CB5662327",
} as const;

/**
 * The ACL owner. The mock never calls `ACL.initializeFromEmptyProxy`; instead it writes this address
 * straight into ACL's Ownable slot (see deploy.ts). This account also sends every
 * `initializeFromEmptyProxy` transaction, so it must be impersonated and funded.
 */
export const ACL_OWNER = "0x0000000000000000000000000000000000000001";

/** `ProtocolConfig.initialKmsNodeParams[0].txSenderAddress`. Arbitrary; must merely be consistent. */
export const KMS_TX_SENDER = "0x0000000000000000000000000000000000C0FFEE";

/**
 * Default mock signing keys. Identical to the ones the Foundry harness bakes in
 * (`host-contracts-cleartext/src/testing/FhevmTest.sol`: MOCK_KMS_SIGNER_PK / MOCK_INPUT_SIGNER_PK),
 * so proofs produced here verify there and vice-versa.
 */
export const KMS_SIGNER_PRIVATE_KEY = "0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91";
export const COPROCESSOR_SIGNER_PRIVATE_KEY = "0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901";

export const KMS_THRESHOLD = 1;
export const INPUT_VERIFIER_THRESHOLD = 1;

/**
 * EIP-712 is CROSS-CHAIN here. The signing DOMAIN uses the gateway chain id and the gateway-side
 * contract address; the message's `contractChainId` field carries the HOST chain id. Mixing the two
 * silently breaks signature recovery — see `encrypt.ts`.
 *
 * (`GATEWAY_INPUT_VERIFICATION_ADDRESS` happens to equal `confidentialBridgeAdd` in
 * FHEVMHostAddresses.sol. Coincidence — different chains, unrelated contracts.)
 */
export const GATEWAY_CHAIN_ID = 10901;
export const GATEWAY_DECRYPTION_ADDRESS = "0x5ffdaAB0373E62E2ea2944776209aEf29E631A64";
export const GATEWAY_INPUT_VERIFICATION_ADDRESS = "0x812b06e1CDCE800494b79fFE4f925A504a9A9810";

export const EIP712_INPUT_VERIFICATION_DOMAIN_NAME = "InputVerification";
export const EIP712_DECRYPTION_DOMAIN_NAME = "Decryption";
export const EIP712_DOMAIN_VERSION = "1";

/**
 * HCU limits passed to `HCULimit.initializeFromEmptyProxy(uint48,uint48,uint48)`.
 *
 * `hcuCapPerBlock` is deliberately the uint48 maximum (2^48-1) rather than the Foundry harness's
 * 20_000_000: a per-block cap throttles longer mock test runs with `HCUBlockLimitExceeded`, which is a
 * production concern, not a testing one. Depth and per-tx limits are kept at the real values so tests
 * still observe the real per-transaction constraints.
 */
export const HCU_CAP_PER_BLOCK = 281_474_976_710_655n; // 2^48 - 1
export const HCU_MAX_DEPTH_PER_TX = 5_000_000n;
export const HCU_MAX_PER_TX = 20_000_000n;

/** erc7201 storage locations (OpenZeppelin upgradeable). Asserted at use site. */
export const INITIALIZABLE_STORAGE_SLOT = "0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00";
export const OWNABLE_STORAGE_SLOT = "0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300";

export interface EngineSigners {
  readonly kms: Wallet;
  readonly coprocessor: Wallet;
}

export function defaultSigners(): EngineSigners {
  return {
    kms: new Wallet(KMS_SIGNER_PRIVATE_KEY),
    coprocessor: new Wallet(COPROCESSOR_SIGNER_PRIVATE_KEY),
  };
}
