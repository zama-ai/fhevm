// Deployment and signing helpers for a REAL Safe v1.4.1 used as the ERC-1271
// smart account in the user-decryption suites.
//
// The contracts are deployed from the prebuilt hardhat artifacts shipped in
// `@safe-global/safe-contracts@1.4.1-2` (`build/artifacts/**`), whose
// `deployedBytecode` is byte-identical to the canonical mainnet Safe v1.4.1
// deployments (keccak256-verified against `@safe-global/safe-deployments`
// codeHashes). Nothing is compiled locally and no TypeChain types exist for
// Safe — interactions go through `ethers.Contract` with the shipped ABIs.
//
// What the pipeline sees is unchanged — an opaque `isValidSignature(digest,
// blob)` STATICCALL — but unlike the retired `ERC1271MultisigWallet` mock, a
// real Safe:
//   - verifies signatures over the SafeMessage EIP-712 RE-HASH of the digest
//     (`getMessageHash(digest)`, domain `{chainId, verifyingContract: proxy}`
//     with NO name/version), never over the raw digest — every signing helper
//     here wraps accordingly, including `approveHash` targets and the eth_sign
//     wrap;
//   - REVERTS on invalid signatures (GS020 short blob / GS025 unapproved
//     hash / GS026 non-owner, duplicate or descending part) instead of
//     returning a non-magic value — an equally definitive rejection for the
//     relayer, the KMS Connector, and the js-sdk;
//   - answers through a proxy -> fallback-handler double round trip. Measured
//     on the canonical bytecode: worst positive shape (3-of-3 ECDSA) needs
//     ~67k gas including intrinsic — comfortably inside the 100_000
//     `erc1271_gas_limit` all three verifying layers apply.
import SafeArtifact from '@safe-global/safe-contracts/build/artifacts/contracts/Safe.sol/Safe.json';
import CompatibilityFallbackHandlerArtifact from '@safe-global/safe-contracts/build/artifacts/contracts/handler/CompatibilityFallbackHandler.sol/CompatibilityFallbackHandler.json';
import SafeProxyFactoryArtifact from '@safe-global/safe-contracts/build/artifacts/contracts/proxies/SafeProxyFactory.sol/SafeProxyFactory.json';
import { Contract, ContractFactory, TypedDataEncoder, ZeroAddress, getBytes, hexlify, randomBytes } from 'ethers';
import type { InterfaceAbi, Signer, TypedDataDomain } from 'ethers';

import { SignaturePart, concatSignatureParts, sortSignatureParts } from '../sdk/unified/unifiedUserDecrypt';

/**
 * Safe v1.4.1 infrastructure shared by every Safe proxy in a suite: the
 * singleton, the CompatibilityFallbackHandler (the ERC-1271 entry point) and
 * the proxy factory. Deploy once per suite in `before()`.
 */
export interface SafeInfra {
  readonly singletonAddress: string;
  readonly handlerAddress: string;
  /** SafeProxyFactory connected to the deployer. */
  readonly factory: Contract;
}

/**
 * A deployed Safe proxy. (Not "handle" — in this repo a handle is a
 * ciphertext handle.)
 */
export interface SafeAccount {
  readonly address: string;
  /** Safe ABI attached to the proxy, connected to the deployer. */
  readonly safe: Contract;
  /** Host chain id captured at deploy time — the SafeMessage domain chainId. */
  readonly chainId: number;
}

/** EIP-712 type of the message a Safe owner signs (`SafeMessage(bytes message)`). */
const SAFE_MESSAGE_TYPES: Record<string, Array<{ name: string; type: string }>> = {
  SafeMessage: [{ name: 'message', type: 'bytes' }],
};

/** Deploy the Safe v1.4.1 singleton, fallback handler and proxy factory from the canonical artifacts. */
export async function deploySafeInfra(deployer: Signer): Promise<SafeInfra> {
  const deployFrom = async (artifact: { abi: unknown; bytecode: string }): Promise<Contract> => {
    const factory = new ContractFactory(artifact.abi as InterfaceAbi, artifact.bytecode, deployer);
    const contract = await factory.deploy();
    await contract.waitForDeployment();
    return contract as Contract;
  };
  const singleton = await deployFrom(SafeArtifact);
  const handler = await deployFrom(CompatibilityFallbackHandlerArtifact);
  const factory = await deployFrom(SafeProxyFactoryArtifact);
  return {
    singletonAddress: await singleton.getAddress(),
    handlerAddress: await handler.getAddress(),
    factory,
  };
}

/**
 * Deploy an M-of-N Safe proxy. The singleton blocks direct `setup()` (its
 * constructor pins `threshold = 1`), so a proxy is the only way to get a
 * usable Safe: `createProxyWithNonce` CREATE2-deploys it and runs `setup()`
 * with the fallback handler wired in — without the handler the proxy has no
 * `isValidSignature` at all. The random salt keeps parallel mocha processes
 * collision-free.
 */
export async function deploySafeAccount(
  infra: SafeInfra,
  deployer: Signer,
  owners: readonly string[],
  threshold: number,
  saltNonce: bigint = BigInt(hexlify(randomBytes(32))),
): Promise<SafeAccount> {
  const safeInterface = new Contract(infra.singletonAddress, SafeArtifact.abi, deployer).interface;
  const initializer = safeInterface.encodeFunctionData('setup', [
    owners,
    threshold,
    ZeroAddress, // to: no setup delegatecall
    '0x', // data
    infra.handlerAddress,
    ZeroAddress, // paymentToken
    0, // payment
    ZeroAddress, // paymentReceiver
  ]);
  const address: string = await infra.factory.createProxyWithNonce.staticCall(
    infra.singletonAddress,
    initializer,
    saltNonce,
  );
  await (await infra.factory.createProxyWithNonce(infra.singletonAddress, initializer, saltNonce)).wait();

  const provider = deployer.provider;
  if (!provider) {
    throw new Error('deploySafeAccount requires a provider-connected deployer');
  }
  const chainId = Number((await provider.getNetwork()).chainId);
  const account: SafeAccount = {
    address,
    safe: new Contract(address, SafeArtifact.abi, deployer),
    chainId,
  };

  // Fail fast on any wiring drift: the local SafeMessage hash must match the
  // handler's on-chain one, and setup() must have taken.
  const handlerView = new Contract(address, CompatibilityFallbackHandlerArtifact.abi, provider);
  const probeDigest = hexlify(randomBytes(32));
  const onChainHash: string = await handlerView.getMessageHash(probeDigest);
  const localHash = safeMessageHashOf(account, probeDigest);
  if (onChainHash !== localHash) {
    throw new Error(`SafeMessage hash mismatch for ${address}: local ${localHash} vs on-chain ${onChainHash}`);
  }
  const onChainThreshold: bigint = await account.safe.getThreshold();
  if (onChainThreshold !== BigInt(threshold)) {
    throw new Error(`Safe ${address} threshold is ${onChainThreshold}, expected ${threshold}`);
  }
  return account;
}

/**
 * The EIP-712 payload a Safe OWNER signs for an ERC-1271 digest. The handler
 * verifies over `getMessageHash(digest)` = EIP-712(`SafeMessage(bytes)`) with
 * `message = abi.encode(digest)` (the raw 32 digest bytes) under the domain
 * `{chainId, verifyingContract: proxy}` — no name, no version.
 */
export function safeMessageTypedData(
  chainId: number,
  safeAddress: string,
  digest: string,
): {
  domain: TypedDataDomain;
  types: Record<string, Array<{ name: string; type: string }>>;
  message: { message: string };
} {
  return {
    domain: { chainId, verifyingContract: safeAddress },
    types: SAFE_MESSAGE_TYPES,
    message: { message: digest },
  };
}

/**
 * The SafeMessage hash of `digest` — the value Safe's `checkSignatures`
 * actually verifies, and therefore the hash `approveHash` must target and the
 * eth_sign flow must wrap.
 */
export function safeMessageHashOf(safe: SafeAccount, digest: string): string {
  const { domain, types, message } = safeMessageTypedData(safe.chainId, safe.address, digest);
  return TypedDataEncoder.hash(domain, types, message);
}

/**
 * One plain-ECDSA (v=27/28) 65-byte part per owner over the SafeMessage wrap
 * of `digest` — UNSORTED; arrange/sort/concat per scenario (see
 * `sortSignatureParts` / `concatSignatureParts`).
 */
export async function collectSafeOwnerParts(
  safe: SafeAccount,
  digest: string,
  owners: readonly Signer[],
): Promise<SignaturePart[]> {
  const { domain, types, message } = safeMessageTypedData(safe.chainId, safe.address, digest);
  return Promise.all(
    owners.map(async (owner) => ({
      address: (await owner.getAddress()).toLowerCase(),
      signature: await owner.signTypedData(domain, types, message),
    })),
  );
}

/**
 * Build a Safe multisig blob for `digest`: one SafeMessage-wrapped ECDSA part
 * per owner, concatenated sorted ascending by owner address (Safe's GS026
 * ordering-doubles-as-dedup rule). `order: 'descending'` deliberately reverses
 * the parts for ordering negatives; passing the same signer twice yields a
 * duplicated-part blob; `trailingHex` appends raw bytes past the static
 * section, which Safe ignores for non-contract (v != 0) parts.
 */
export async function buildSafeMultisigSignature(
  safe: SafeAccount,
  digest: string,
  owners: readonly Signer[],
  opts?: { readonly order?: 'ascending' | 'descending'; readonly trailingHex?: string },
): Promise<string> {
  const parts = sortSignatureParts(await collectSafeOwnerParts(safe, digest, owners));
  if (opts?.order === 'descending') {
    parts.reverse();
  }
  return concatSignatureParts(parts, opts?.trailingHex ?? '');
}

/**
 * An eth_sign-flavored part: the owner `personal_sign`s the 32 raw bytes of
 * the SafeMessage hash, and the recovery byte is shifted by 4 (0x1b -> 0x1f,
 * 0x1c -> 0x20) — Safe's `v > 30` branch un-shifts and verifies over the
 * `\x19Ethereum Signed Message:` wrap of the SafeMessage hash.
 */
export async function safeEthSignPart(safe: SafeAccount, digest: string, owner: Signer): Promise<SignaturePart> {
  const raw = await owner.signMessage(getBytes(safeMessageHashOf(safe, digest)));
  const v = parseInt(raw.slice(-2), 16) + 4;
  return {
    address: (await owner.getAddress()).toLowerCase(),
    signature: raw.slice(0, -2) + v.toString(16),
  };
}

/**
 * A pre-approved-hash part (Safe's `v == 1` type): `r` carries the approving
 * owner's address, `s` is unused, and no key ever signs. Valid only after
 * `approveSafeHash` recorded the approval on-chain — inside the ERC-1271 path
 * `msg.sender` is the fallback handler, so Safe's "sender is the owner"
 * shortcut can never apply.
 */
export function safeApprovedHashPart(ownerAddress: string): SignaturePart {
  return {
    address: ownerAddress.toLowerCase(),
    signature: `0x${ownerAddress.slice(2).toLowerCase().padStart(64, '0')}${'0'.repeat(64)}01`,
  };
}

/**
 * Record the on-chain approval backing a `safeApprovedHashPart`: the owner
 * calls `approveHash` with the SafeMessage hash of `digest` — NOT the raw
 * digest, which is what the retired mock approved.
 */
export async function approveSafeHash(safe: SafeAccount, owner: Signer, digest: string): Promise<void> {
  await (await (safe.safe.connect(owner) as Contract).approveHash(safeMessageHashOf(safe, digest))).wait();
}
