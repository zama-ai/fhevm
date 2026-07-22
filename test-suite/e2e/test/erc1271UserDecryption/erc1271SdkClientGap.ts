import { expect } from 'chai';
import type { Signer } from 'ethers';
import { getAddress } from 'ethers';
import { ethers } from 'hardhat';

import { ERC1271ApproveHashWallet, ERC1271MultisigWallet, EncryptedValueHolder } from '../../types';
import { createInstances } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import type { SignaturePart } from '../sdk/unified/unifiedUserDecrypt';
import { concatSignatureParts, sortSignatureParts } from '../sdk/unified/unifiedUserDecrypt';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

// The SDK-client leg of the ERC-1271 user-decryption chain
// (SDK -> Relayer -> Gateway -> KMS Connector -> isValidSignature).
//
// The protocol backend accepts variable-length ERC-1271 signatures — proven
// end to end by erc1271UserDecryption.ts, which POSTs the /v3 envelope
// directly. These tests drive the SAME scenarios through the @fhevm/sdk
// client via its permit-injection surface (`parseSignedDecryptionPermit`
// with an externally assembled signature) followed by `decryptValue`, and
// assert the decrypted plaintext.
//
// Decrypt shape: the encrypted handle lives on a separate holder contract
// with `FHE.allow(handle, wallet)`, and the wallet is only the userAddress —
// the realistic setup (a wallet holding confidential tokens), and the only
// one expressible through the SDK: both the SDK (`checkPersistAllowed`) and
// the KMS connector reject `userAddress == contractAddress` /
// `userAddress` listed in `allowedContracts`. The wallet-holds-its-own-handle
// shape stays covered by the raw-envelope suite via permissive
// `allowedContracts: []`.
//
// >>> EXPECTED TO FAIL until the SDK supports ERC-1271 signatures. <<<
// Current failure points (the SDK adaptation tracked by devx should clear
// them, making this suite the acceptance test):
//  - tests 1 and 3 throw at the permit parse: the signature is
//    runtime-asserted to be exactly 65 bytes (`Bytes65Hex`,
//    core/kms/SignedDecryptionPermitV2-p.ts / core/base/bytes.ts) — a
//    130-byte multisig blob and the empty `0x` approved-hash signature are
//    both rejected client-side, before any network call;
//  - test 2 throws at the client-side verification: it is ECDSA-recover-only
//    and requires recovered == signerAddress
//    (core/utils-p/decrypt/verifyKmsUserDecryptEip712V2.ts) — there is no
//    isValidSignature branch for a contract-wallet userAddress, so even a
//    well-formed 65-byte owner signature is rejected locally.
// The relayer's synchronous ERC-1271 pre-check (400 on a definitively-bad
// signature) is the authority the client can defer to once those gates are
// adapted.
//
// The suite skips itself when the legacy `@zama-fhe/relayer-sdk` adapter is
// active (RELAYER_SDK_VERSION set): the surface under test is @fhevm/sdk's.

const KNOWN_VALUE = 123456789n;
const POSITIVE_TIMEOUT_MS = 3 * 60 * 1000;
const TIMEOUT_MARGIN_MS = 60 * 1000;

/** The EIP-712 struct types WITHOUT EIP712Domain — the shape ethers' `signTypedData` expects. */
type StructTypes = Record<string, Array<{ name: string; type: string }>>;

/** Minimal mutable view of the permit's eip712 payload used to craft wallet-userAddress permits. */
interface MutableEip712 {
  domain: Record<string, unknown>;
  types: StructTypes;
  primaryType: string;
  message: Record<string, unknown> & { userAddress: string };
}

describe('ERC-1271 user decryption SDK client', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let sdk: FhevmSdk;
  let client: FhevmSdk['rawClient'];
  let transportKeyPair: Awaited<ReturnType<FhevmSdk['rawClient']['generateTransportKeyPair']>>;

  let multisig2of3: ERC1271MultisigWallet;
  let multisig2of3Address: string;
  let multisig1of3: ERC1271MultisigWallet;
  let multisig1of3Address: string;
  let approveWallet: ERC1271ApproveHashWallet;
  let approveWalletAddress: string;
  /** One holder per wallet: the dapp-side contract carrying that wallet's handle. */
  let holders: Map<string, { holder: EncryptedValueHolder; holderAddress: string }>;

  before(async function () {
    this.timeout(240_000);
    await initSigners(5);
    signers = await getSigners();
    instances = await createInstances(signers);

    if (!(instances.alice instanceof FhevmSdk)) {
      // Legacy @zama-fhe/relayer-sdk adapter active — the surface under test is @fhevm/sdk's.
      this.skip();
    }
    sdk = instances.alice;
    client = sdk.rawClient;
    transportKeyPair = await client.generateTransportKeyPair();

    // Owners bob/carol/dave. The 1-of-3 wallet pins the Safe-threshold-1 case:
    // a single 65-byte part that ecrecover CAN parse (recovering the owner,
    // not the wallet) must still be verified via isValidSignature.
    const multisigFactory = await ethers.getContractFactory('ERC1271MultisigWallet');
    const owners = [signers.bob.address, signers.carol.address, signers.dave.address];
    multisig2of3 = await multisigFactory.connect(signers.alice).deploy(owners, 2);
    await multisig2of3.waitForDeployment();
    multisig2of3Address = await multisig2of3.getAddress();

    multisig1of3 = await multisigFactory.connect(signers.alice).deploy(owners, 1);
    await multisig1of3.waitForDeployment();
    multisig1of3Address = await multisig1of3.getAddress();

    const approveFactory = await ethers.getContractFactory('ERC1271ApproveHashWallet');
    approveWallet = await approveFactory.connect(signers.alice).deploy(signers.bob.address);
    await approveWallet.waitForDeployment();
    approveWalletAddress = await approveWallet.getAddress();

    // One value holder per wallet, granting THAT wallet decrypt access.
    const holderFactory = await ethers.getContractFactory('EncryptedValueHolder');
    holders = new Map();
    for (const walletAddress of [multisig2of3Address, multisig1of3Address, approveWalletAddress]) {
      const holder = await holderFactory.connect(signers.alice).deploy();
      await holder.waitForDeployment();
      const holderAddress = await holder.getAddress();
      await (await holder.connect(signers.alice).initValueFor(KNOWN_VALUE, walletAddress)).wait();
      holders.set(walletAddress, { holder, holderAddress });
    }
  });

  function holderOf(walletAddress: string): { holder: EncryptedValueHolder; holderAddress: string } {
    const entry = holders.get(walletAddress);
    if (!entry) {
      throw new Error(`no holder deployed for wallet ${walletAddress}`);
    }
    return entry;
  }

  /**
   * The SDK's exact EIP-712 permit payload for a wallet userAddress: sign a
   * legitimate self-permit (bob) against the wallet's HOLDER contract — the
   * authoritative source of the SDK's domain/types/message encoding, with the
   * holder in `allowedContracts` — then re-point `message.userAddress` at the
   * wallet.
   */
  async function walletEip712(walletAddress: string): Promise<MutableEip712> {
    const { holderAddress } = holderOf(walletAddress);
    const selfPermit = await client.signDecryptionPermit({
      contractAddresses: [holderAddress as `0x${string}`],
      durationSeconds: 7 * 24 * 3600,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer: signers.bob,
      signerAddress: signers.bob.address as `0x${string}`,
    });
    const eip712 = structuredClone(selfPermit.eip712) as unknown as MutableEip712;
    eip712.message.userAddress = getAddress(walletAddress);
    return eip712;
  }

  /** One owner's plain-ECDSA 65-byte part over the permit's EIP-712 payload. */
  async function ownerPart(eip712: MutableEip712, owner: Signer): Promise<SignaturePart> {
    const { EIP712Domain: _domain, ...structTypes } = eip712.types;
    return {
      address: (await owner.getAddress()).toLowerCase(),
      signature: await owner.signTypedData(
        eip712.domain as Parameters<Signer['signTypedData']>[0],
        structTypes,
        eip712.message,
      ),
    };
  }

  /** Parse an externally assembled wallet permit, then decrypt the wallet's handle on its holder. */
  async function decryptThroughSdk(parameters: {
    readonly eip712: MutableEip712;
    readonly signature: string;
    readonly walletAddress: string;
  }): Promise<bigint> {
    const { holder, holderAddress } = holderOf(parameters.walletAddress);
    const signedPermit = await client.parseSignedDecryptionPermit({
      serializedPermit: {
        version: 2,
        eip712: parameters.eip712 as never,
        signature: parameters.signature,
        signerAddress: getAddress(parameters.walletAddress),
      },
      transportKeyPair,
    });
    const res = await client.decryptValue({
      contractAddress: holderAddress as `0x${string}`,
      transportKeyPair,
      signedPermit,
      encryptedValue: (await holder.value()) as `0x${string}`,
    });
    return typeof res.value === 'number' ? BigInt(res.value) : (res.value as bigint);
  }

  it('test erc1271 sdk client decrypts with a multisig 2-of-3 concatenated signature', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const eip712 = await walletEip712(multisig2of3Address);
    const parts = sortSignatureParts([await ownerPart(eip712, signers.bob), await ownerPart(eip712, signers.carol)]);
    const signature = concatSignatureParts(parts);
    // The whole point: a 130-byte opaque blob through the SDK client.
    expect(signature.length).to.equal(2 + 130 * 2);
    const clear = await decryptThroughSdk({ eip712, signature, walletAddress: multisig2of3Address });
    expect(clear).to.equal(KNOWN_VALUE);
  });

  it('test erc1271 sdk client decrypts with a single-owner signature for a threshold-1 wallet', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const eip712 = await walletEip712(multisig1of3Address);
    // Exactly 65 bytes: ecrecover parses it and recovers bob — NOT the wallet
    // — so verification must go through the wallet's isValidSignature
    // (the Safe-threshold-1 case), not require recovered == userAddress.
    const part = await ownerPart(eip712, signers.bob);
    expect(part.signature.length).to.equal(2 + 130);
    const clear = await decryptThroughSdk({
      eip712,
      signature: part.signature,
      walletAddress: multisig1of3Address,
    });
    expect(clear).to.equal(KNOWN_VALUE);
  });

  it('test erc1271 sdk client decrypts with an empty signature after approveHash', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS + TIMEOUT_MARGIN_MS);
    const eip712 = await walletEip712(approveWalletAddress);
    // Pre-approve the exact EIP-712 digest on-chain, then decrypt with the
    // empty signature — the wallet's pre-approved-hash flow, already
    // supported by the relayer and KMS connector.
    const { EIP712Domain: _domain, ...structTypes } = eip712.types;
    const digest = ethers.TypedDataEncoder.hash(
      eip712.domain as Parameters<(typeof ethers.TypedDataEncoder)['hash']>[0],
      structTypes,
      eip712.message,
    );
    await (await approveWallet.connect(signers.bob).approveHash(digest)).wait();
    const clear = await decryptThroughSdk({ eip712, signature: '0x', walletAddress: approveWalletAddress });
    expect(clear).to.equal(KNOWN_VALUE);
  });
});
