import { Wallet } from "ethers";

/**
 * The values the stack is bootstrapped with.
 *
 * Everything here is a CHOICE, unlike the addresses in `addresses.ts`, three of which are forced on us by
 * `ZamaConfig`. The constraint on these is only self-consistency: the engine initializes the contracts with
 * them (`deploy.ts`) and the FHE layer signs against them (`fhe/encrypt.ts`, `fhe/decrypt.ts`), so the two
 * must agree. They are otherwise arbitrary.
 */

/**
 * ACL's owner. the engine roots the whole stack's ownership in ACL (`ACLOwnable` resolves every other contract's
 * owner through `ACL.owner()`), and `ACL.initializeFromEmptyProxy()` runs `__Ownable_init(owner())` — it
 * PRESERVES the owner already in the slot rather than taking one as an argument. In a real deploy that owner
 * arrives via `EmptyUUPSProxyACL.initialize(deployer)`; here `deploy.ts` writes the Ownable slot directly.
 * This account also sends every `initializeFromEmptyProxy`, so it is impersonated and funded.
 */
export const ACL_OWNER = "0x0000000000000000000000000000000000000001";

/** `ProtocolConfig.initialKmsNodes[0].txSenderAddress`. Arbitrary; must merely be consistent. */
export const KMS_TX_SENDER = "0x0000000000000000000000000000000000C0FFEE";

/** Mock signing keys. Arbitrary, but the coprocessor key must match `InputVerifier`'s initial signer set. */
export const KMS_SIGNER_PRIVATE_KEY = "0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91";
export const COPROCESSOR_SIGNER_PRIVATE_KEY = "0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901";

export const KMS_THRESHOLD = 1n;
export const INPUT_VERIFIER_THRESHOLD = 1n;

/**
 * EIP-712 here is CROSS-CHAIN. `InputVerifier` and `KMSVerifier` are each initialized with a
 * `(verifyingContractSource, chainIDSource)` pair — the gateway-side identity that signatures are
 * domain-separated by. In the current engine these are INIT ARGUMENTS, not baked-in constants, so the values below are
 * simply what we choose to initialize with; the only requirement is that we sign with the same ones.
 *
 * The two domains differ by name, and by which chain id applies per operation:
 *   - InputVerifier -> name "InputVerification", domain chainId = GATEWAY_CHAIN_ID.
 *   - KMSVerifier   -> name "Decryption". Public decrypt uses the GATEWAY chain id; USER decrypt overrides
 *     it to the HOST chain id (`CleartextKMSVerifier._domainHashWithHostChainId`). Mixing the two is a
 *     silent signature-recovery failure — see `fhe/decrypt.ts`.
 */
export const GATEWAY_CHAIN_ID = 10901n;
export const GATEWAY_DECRYPTION_ADDRESS = "0x5ffdaAB0373E62E2ea2944776209aEf29E631A64";
export const GATEWAY_INPUT_VERIFICATION_ADDRESS = "0x812b06e1CDCE800494b79fFE4f925A504a9A9810";

export const EIP712_INPUT_VERIFICATION_DOMAIN_NAME = "InputVerification";
export const EIP712_DECRYPTION_DOMAIN_NAME = "Decryption";
export const EIP712_DOMAIN_VERSION = "1";

/**
 * `HCULimit.initializeFromEmptyProxy(uint48,uint48,uint48)`.
 *
 * `hcuCapPerBlock` is deliberately the uint48 maximum rather than a realistic value: a per-block cap
 * throttles longer test runs with `HCUBlockLimitExceeded`, which is a production concern, not a testing one.
 * Depth and per-tx limits stay at real values so tests still observe the real per-transaction constraints.
 */
export const HCU_CAP_PER_BLOCK = 281_474_976_710_655n; // 2^48 - 1
export const HCU_MAX_DEPTH_PER_TX = 5_000_000n;
export const HCU_MAX_PER_TX = 20_000_000n;

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
