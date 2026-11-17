/**
 * Makes the slot's test key the active key and CRS of the KMSGeneration deployed
 * on a cleartext anvil.
 *
 * The cleartext stack deploys a real KMSGeneration, but no key generation runs on
 * it, so it has no active key and the SDK (which checks every FHE public key and
 * CRS against KMSGeneration before using it) would refuse the slot's key. This
 * runs the contract's own key and CRS generation flows instead:
 *
 *   - the ACL owner requests a keygen and a crsgen;
 *   - the KMS nodes answer them (`prepKeygenResponse`, `keygenResponse`,
 *     `crsgenResponse`) with the digests from `test/keys/key.<ver>.digests.json`,
 *     signed by their KMS signers, until the kmsGen threshold is reached.
 *
 * The KMS signer keys derive from the mnemonic the deploy registers them from
 * (see contracts/scripts/localcleartext/LocalHostBootstrap.sol). The ACL owner and
 * the KMS tx senders are impersonated (anvil cheats), so they need no key.
 *
 * Idempotent: an anvil whose active key and CRS already carry these digests (a
 * reused anvil) is left as is.
 */

import { readFileSync } from 'node:fs';
import {
  createPublicClient,
  createTestClient,
  createWalletClient,
  http,
  parseAbi,
  parseEventLogs,
  type Address,
  type Hex,
  type PublicClient,
  type TransactionReceipt,
} from 'viem';
import { mnemonicToAccount, type HDAccount } from 'viem/accounts';

export type KmsGenerationSeedConfig = {
  /** The KMSGeneration address on the anvil. */
  readonly address: string;
  /** The ProtocolConfig address on the anvil (KMS nodes and threshold). */
  readonly protocolConfigAddress: string;
  /** The ACL address on the anvil (its owner requests the generations). */
  readonly aclAddress: string;
  /** Path to the `test/keys/key.<ver>.digests.json` file of the slot's key. */
  readonly digestsFilePath: string;
};

/**
 * Mnemonic the cleartext deploy derives its KMS signers from, at m/44'/60'/0'/3/i
 * (the SDK's cleartext relayer signs with the same keys).
 */
const KMS_SIGNER_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const KMS_SIGNER_CHANGE_INDEX = 3;
/** How many KMS signer keys to derive when matching the on-chain signers. */
const KMS_SIGNER_POOL_SIZE = 20;
/** Max bit length of the test CRS (the SDK only accepts a 2048-bit CRS). */
const CRS_MAX_BIT_LENGTH = 2048n;
/** `ParamsType.Default`. */
const PARAMS_TYPE_DEFAULT = 0;
/** `KeyType.Public` in KMSGeneration's key digests. */
const KEY_TYPE_PUBLIC = 1;
/** Gas money for the impersonated senders (anvil_setBalance). */
const SENDER_BALANCE = 10n ** 21n;

const ACL_ABI = parseAbi(['function owner() view returns (address)']);

const PROTOCOL_CONFIG_ABI = parseAbi([
  'struct KmsNode { address txSenderAddress; address signerAddress; string ipAddress; string storageUrl; }',
  'function getCurrentKmsContextId() view returns (uint256)',
  'function getKmsNodesForContext(uint256 kmsContextId) view returns (KmsNode[])',
  'function getKmsGenThresholdForContext(uint256 kmsContextId) view returns (uint256)',
]);

const KMS_GENERATION_ABI = parseAbi([
  'struct KeyDigest { uint8 keyType; bytes digest; }',
  'function keygen(uint8 paramsType, uint256 existingKeyId)',
  'function prepKeygenResponse(uint256 prepKeygenId, bytes signature)',
  'function keygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes signature)',
  'function crsgenRequest(uint256 maxBitLength, uint8 paramsType)',
  'function crsgenResponse(uint256 crsId, bytes crsDigest, bytes signature)',
  'function getActiveKeyId() view returns (uint256)',
  'function getActiveCrsId() view returns (uint256)',
  'function getKeyMaterials(uint256 keyId) view returns (string[], KeyDigest[])',
  'function getCrsMaterials(uint256 crsId) view returns (string[], bytes)',
  'event PrepKeygenRequest(uint256 prepKeygenId, uint8 paramsType, uint256 existingKeyId, bytes extraData)',
  'event KeygenRequest(uint256 prepKeygenId, uint256 keyId, uint256 existingKeyId, bytes extraData)',
  'event CrsgenRequest(uint256 crsId, uint256 maxBitLength, uint8 paramsType, bytes extraData)',
]);

const EIP712_TYPES = {
  PrepKeygenVerification: [
    { name: 'prepKeygenId', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
  KeygenVerification: [
    { name: 'prepKeygenId', type: 'uint256' },
    { name: 'keyId', type: 'uint256' },
    { name: 'keyDigests', type: 'KeyDigest[]' },
    { name: 'extraData', type: 'bytes' },
  ],
  KeyDigest: [
    { name: 'keyType', type: 'uint8' },
    { name: 'digest', type: 'bytes' },
  ],
  CrsgenVerification: [
    { name: 'crsId', type: 'uint256' },
    { name: 'maxBitLength', type: 'uint256' },
    { name: 'crsDigest', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

type Digests = { readonly publicKeyDigest: Hex; readonly crsDigest: Hex };

/** A KMS node that answers the generation requests: its tx sender and signer key. */
type KmsResponder = { readonly txSender: Address; readonly signer: HDAccount };

/**
 * Activates the slot key's digests in the anvil's KMSGeneration (see the file
 * header). Returns `false` when they were already active (nothing was sent).
 */
export async function seedKmsGeneration(rpcUrl: string, config: KmsGenerationSeedConfig): Promise<boolean> {
  const digests = JSON.parse(readFileSync(config.digestsFilePath, 'utf-8')) as Digests;
  const address = config.address as Address;
  const publicClient = createPublicClient({ transport: http(rpcUrl) });

  const keyActive = await _isActiveKey(publicClient, address, digests.publicKeyDigest);
  const crsActive = await _isActiveCrs(publicClient, address, digests.crsDigest);
  if (keyActive && crsActive) {
    return false;
  }

  const chainId = await publicClient.getChainId();
  const domain = { name: 'KMSGeneration', version: '1', chainId, verifyingContract: address } as const;
  const sender = new _Sender(rpcUrl, publicClient);
  const owner = await publicClient.readContract({
    address: config.aclAddress as Address,
    abi: ACL_ABI,
    functionName: 'owner',
  });
  const responders = await _kmsResponders(publicClient, config.protocolConfigAddress as Address);

  if (!keyActive) {
    const keyDigests = [{ keyType: KEY_TYPE_PUBLIC, digest: digests.publicKeyDigest }] as const;

    const keygenTx = await sender.send(owner, address, 'keygen', [PARAMS_TYPE_DEFAULT, 0n]);
    const prep = _required(
      parseEventLogs({ abi: KMS_GENERATION_ABI, eventName: 'PrepKeygenRequest', logs: keygenTx.logs })[0]?.args,
      'PrepKeygenRequest',
      keygenTx,
    );
    let keygen: { readonly prepKeygenId: bigint; readonly keyId: bigint; readonly extraData: Hex } | undefined;
    for (const r of responders) {
      const signature = await r.signer.signTypedData({
        domain,
        types: EIP712_TYPES,
        primaryType: 'PrepKeygenVerification',
        message: { prepKeygenId: prep.prepKeygenId, extraData: prep.extraData },
      });
      const receipt = await sender.send(r.txSender, address, 'prepKeygenResponse', [prep.prepKeygenId, signature]);
      keygen ??= parseEventLogs({ abi: KMS_GENERATION_ABI, eventName: 'KeygenRequest', logs: receipt.logs })[0]?.args;
    }
    if (keygen === undefined) {
      throw new Error(`KMSGeneration ${address}: no consensus on prepKeygen ${prep.prepKeygenId.toString(16)}.`);
    }

    for (const r of responders) {
      const signature = await r.signer.signTypedData({
        domain,
        types: EIP712_TYPES,
        primaryType: 'KeygenVerification',
        message: { prepKeygenId: keygen.prepKeygenId, keyId: keygen.keyId, keyDigests, extraData: keygen.extraData },
      });
      await sender.send(r.txSender, address, 'keygenResponse', [keygen.keyId, keyDigests, signature]);
    }
    if (!(await _isActiveKey(publicClient, address, digests.publicKeyDigest))) {
      throw new Error(`KMSGeneration ${address}: key ${keygen.keyId.toString(16)} was not activated.`);
    }
  }

  if (!crsActive) {
    const crsgenTx = await sender.send(owner, address, 'crsgenRequest', [CRS_MAX_BIT_LENGTH, PARAMS_TYPE_DEFAULT]);
    const crsgen = _required(
      parseEventLogs({ abi: KMS_GENERATION_ABI, eventName: 'CrsgenRequest', logs: crsgenTx.logs })[0]?.args,
      'CrsgenRequest',
      crsgenTx,
    );
    for (const r of responders) {
      const signature = await r.signer.signTypedData({
        domain,
        types: EIP712_TYPES,
        primaryType: 'CrsgenVerification',
        message: {
          crsId: crsgen.crsId,
          maxBitLength: crsgen.maxBitLength,
          crsDigest: digests.crsDigest,
          extraData: crsgen.extraData,
        },
      });
      await sender.send(r.txSender, address, 'crsgenResponse', [crsgen.crsId, digests.crsDigest, signature]);
    }
    if (!(await _isActiveCrs(publicClient, address, digests.crsDigest))) {
      throw new Error(`KMSGeneration ${address}: CRS ${crsgen.crsId.toString(16)} was not activated.`);
    }
  }
  return true;
}

/** Whether the active key's `Public` digest is `digest`. */
async function _isActiveKey(client: PublicClient, address: Address, digest: Hex): Promise<boolean> {
  const keyId = await client.readContract({ address, abi: KMS_GENERATION_ABI, functionName: 'getActiveKeyId' });
  if (keyId === 0n) {
    return false;
  }
  const [, keyDigests] = await client.readContract({
    address,
    abi: KMS_GENERATION_ABI,
    functionName: 'getKeyMaterials',
    args: [keyId],
  });
  return keyDigests.some((d) => d.keyType === KEY_TYPE_PUBLIC && d.digest.toLowerCase() === digest.toLowerCase());
}

/** Whether the active CRS's digest is `digest`. */
async function _isActiveCrs(client: PublicClient, address: Address, digest: Hex): Promise<boolean> {
  const crsId = await client.readContract({ address, abi: KMS_GENERATION_ABI, functionName: 'getActiveCrsId' });
  if (crsId === 0n) {
    return false;
  }
  const [, crsDigest] = await client.readContract({
    address,
    abi: KMS_GENERATION_ABI,
    functionName: 'getCrsMaterials',
    args: [crsId],
  });
  return crsDigest.toLowerCase() === digest.toLowerCase();
}

/**
 * The KMS nodes of the current context whose signer key we can derive, as many
 * as the kmsGen threshold needs.
 */
async function _kmsResponders(client: PublicClient, protocolConfig: Address): Promise<KmsResponder[]> {
  const contextId = await client.readContract({
    address: protocolConfig,
    abi: PROTOCOL_CONFIG_ABI,
    functionName: 'getCurrentKmsContextId',
  });
  const [nodes, threshold] = await Promise.all([
    client.readContract({
      address: protocolConfig,
      abi: PROTOCOL_CONFIG_ABI,
      functionName: 'getKmsNodesForContext',
      args: [contextId],
    }),
    client.readContract({
      address: protocolConfig,
      abi: PROTOCOL_CONFIG_ABI,
      functionName: 'getKmsGenThresholdForContext',
      args: [contextId],
    }),
  ]);

  const signers = new Map<string, HDAccount>();
  for (let i = 0; i < KMS_SIGNER_POOL_SIZE; i++) {
    const account = mnemonicToAccount(KMS_SIGNER_MNEMONIC, { changeIndex: KMS_SIGNER_CHANGE_INDEX, addressIndex: i });
    signers.set(account.address.toLowerCase(), account);
  }

  const responders: KmsResponder[] = [];
  for (const node of nodes) {
    const signer = signers.get(node.signerAddress.toLowerCase());
    if (signer !== undefined) {
      responders.push({ txSender: node.txSenderAddress, signer });
    }
  }
  if (BigInt(responders.length) < threshold) {
    throw new Error(
      `ProtocolConfig ${protocolConfig}: kmsGen threshold is ${threshold.toString()}, but only ` +
        `${String(responders.length)} of the ${String(nodes.length)} KMS signers derive from the test mnemonic.`,
    );
  }
  return responders.slice(0, Number(threshold));
}

type KmsGenerationFunction = 'keygen' | 'prepKeygenResponse' | 'keygenResponse' | 'crsgenRequest' | 'crsgenResponse';

/** Sends KMSGeneration transactions from impersonated accounts. */
class _Sender {
  readonly #rpcUrl: string;
  readonly #publicClient: PublicClient;
  readonly #impersonated = new Set<string>();

  constructor(rpcUrl: string, publicClient: PublicClient) {
    this.#rpcUrl = rpcUrl;
    this.#publicClient = publicClient;
  }

  async send(
    from: Address,
    to: Address,
    functionName: KmsGenerationFunction,
    args: readonly unknown[],
  ): Promise<TransactionReceipt> {
    await this.#impersonate(from);
    const wallet = createWalletClient({ account: from, transport: http(this.#rpcUrl) });
    const hash = await wallet.writeContract({
      chain: null,
      address: to,
      abi: KMS_GENERATION_ABI,
      functionName,
      args,
    } as Parameters<typeof wallet.writeContract>[0]);
    const receipt = await this.#publicClient.waitForTransactionReceipt({ hash });
    if (receipt.status !== 'success') {
      throw new Error(`KMSGeneration.${functionName} from ${from} reverted (tx ${hash}).`);
    }
    return receipt;
  }

  async #impersonate(account: Address): Promise<void> {
    if (this.#impersonated.has(account)) {
      return;
    }
    const testClient = createTestClient({ mode: 'anvil', transport: http(this.#rpcUrl) });
    await testClient.impersonateAccount({ address: account });
    await testClient.setBalance({ address: account, value: SENDER_BALANCE });
    this.#impersonated.add(account);
  }
}

/** `args` (an event's args parsed from `receipt`), or throws when the event is missing. */
function _required<T>(args: T | undefined, eventName: string, receipt: TransactionReceipt): T {
  if (args === undefined) {
    throw new Error(`No ${eventName} event in tx ${receipt.transactionHash}.`);
  }
  return args;
}
