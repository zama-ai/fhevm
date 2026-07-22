import { expect } from 'chai';
import { TypedDataEncoder, getAddress } from 'ethers';
import type { Signer } from 'ethers';
import { ethers } from 'hardhat';

import { ERC1271ApproveHashWallet, ERC1271MultisigWallet } from '../../types';
import { createInstances } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

// End-to-end coverage of `@fhevm/sdk` ERC-1271 (smart-contract-wallet) support
// on the unified /v3 user-decryption route.
//
// The protocol backend (relayer /v3 -> gateway -> KMS connector) accepts
// variable-length ERC-1271 signatures — also proven by the direct-envelope
// tests in erc1271UserDecryption.ts. This suite drives the SAME flows THROUGH
// the SDK client, using only its EXISTING public surface — there is no new
// public method and no `signatureKind` discriminator:
//
//   gate 1 — signature shape: `parseSignedDecryptionPermit` accepts a
//            variable-length blob (concatenated multisig, or the empty `0x`
//            pre-approved-hash flow); a normal EOA permit still uses the strict
//            65-byte shape.
//   gate 2 — client-side verification: the permit is checked against
//            `eip712.message.userAddress` and AUTO-DETECTS EOA vs ERC-1271 —
//            a 65-byte EOA fast-path that recovers to `userAddress` returns
//            before any RPC, otherwise it falls through to an
//            `isValidSignature` STATICCALL (precautionary; the KMS is
//            authoritative).
//
// A smart-contract-wallet permit is issued by pointing the serialized permit's
// `eip712.message.userAddress` at the wallet and passing the assembled blob to
// `parseSignedDecryptionPermit`. The signing path (`signDecryptionPermit`) is
// deliberately unchanged and stays EOA/self-only: it hard-wires `userAddress`
// to the connected signer (asserted below).
//
// The suite skips itself when the legacy `@zama-fhe/relayer-sdk` adapter is
// active (RELAYER_SDK_VERSION set): the surface exercised here is @fhevm/sdk's.

const KNOWN_VALUE = 123456789n;
const DURATION_SECONDS = 7 * 24 * 3600;

/** The EIP-712 struct types WITHOUT EIP712Domain — the shape ethers' `signTypedData` expects. */
type StructTypes = Record<string, Array<{ name: string; type: string }>>;

/** Minimal mutable view of the permit's eip712 payload used to craft wallet-userAddress variants. */
interface MutableEip712 {
  domain: Record<string, unknown>;
  types: StructTypes;
  primaryType: string;
  message: Record<string, unknown> & { userAddress: string };
}

/** Await a promise and return the error it rejects with; fails the test if it resolves. */
async function captureRejection(promise: Promise<unknown>, label: string): Promise<Error> {
  try {
    await promise;
  } catch (err) {
    return err as Error;
  }
  throw new Error(`${label} unexpectedly succeeded — expected a definitive ERC-1271 rejection.`);
}

describe('ERC-1271 user decryption via the SDK client', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let sdk: FhevmSdk;
  let client: FhevmSdk['rawClient'];
  let transportKeyPair: Awaited<ReturnType<FhevmSdk['rawClient']['generateTransportKeyPair']>>;

  let multisigWallet: ERC1271MultisigWallet;
  let multisigWalletAddress: string;
  let approveWallet: ERC1271ApproveHashWallet;
  let approveWalletAddress: string;

  /**
   * A permit legitimately signed by bob for HIMSELF, used as the authoritative
   * template for the SDK's exact EIP-712 shape (domain incl. chainId and
   * verifying contract, struct types, message field encoding). The gap tests
   * clone it and re-point `message.userAddress` at a wallet.
   */
  let templateEip712: MutableEip712;

  // Captured once so the multisig parts (signed over the template's EIP-712)
  // match the digest the SDK rebuilds from the same parameters (the
  // minute-rounded startTimestamp in particular).
  let startTimestamp: number;

  before(async function () {
    this.timeout(180_000);
    await initSigners(5);
    signers = await getSigners();
    instances = await createInstances(signers);

    if (!(instances.alice instanceof FhevmSdk)) {
      // Legacy @zama-fhe/relayer-sdk adapter active — the gap pinned here is @fhevm/sdk's.
      this.skip();
    }
    sdk = instances.alice;
    client = sdk.rawClient;
    transportKeyPair = await client.generateTransportKeyPair();

    // 2-of-3 multisig wallet (owners bob/carol/dave) and a Safe-style
    // approveHash wallet (owner bob) — the userAddress targets of the tests
    // and, via initValue, the handles for the commented happy paths.
    const multisigFactory = await ethers.getContractFactory('ERC1271MultisigWallet');
    multisigWallet = await multisigFactory
      .connect(signers.alice)
      .deploy([signers.bob.address, signers.carol.address, signers.dave.address], 2);
    await multisigWallet.waitForDeployment();
    multisigWalletAddress = await multisigWallet.getAddress();
    await (await multisigWallet.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    const approveFactory = await ethers.getContractFactory('ERC1271ApproveHashWallet');
    approveWallet = await approveFactory.connect(signers.alice).deploy(signers.bob.address);
    await approveWallet.waitForDeployment();
    approveWalletAddress = await approveWallet.getAddress();
    await (await approveWallet.connect(signers.alice).initValue(KNOWN_VALUE)).wait();

    startTimestamp = Math.floor(Date.now() / 1000);

    const legitPermit = await client.signDecryptionPermit({
      contractAddresses: [multisigWalletAddress],
      durationSeconds: DURATION_SECONDS,
      startTimestamp,
      transportKeyPair,
      signer: signers.bob,
      signerAddress: signers.bob.address as `0x${string}`,
    });
    templateEip712 = structuredClone(legitPermit.eip712) as unknown as MutableEip712;
  });

  /** Clone the template eip712 and re-point its userAddress at `walletAddress`. */
  function eip712ForWallet(walletAddress: string): MutableEip712 {
    const eip712 = structuredClone(templateEip712);
    eip712.message.userAddress = getAddress(walletAddress);
    return eip712;
  }

  /** The struct types without EIP712Domain, as ethers' signTypedData expects. */
  function structTypesOf(eip712: MutableEip712): StructTypes {
    const { EIP712Domain: _domain, ...structTypes } = eip712.types;
    return structTypes;
  }

  /** One owner's 65-byte ECDSA part over the permit's EIP-712 payload. */
  async function ownerPart(eip712: MutableEip712, owner: Signer): Promise<{ address: string; signature: string }> {
    const address = (await owner.getAddress()).toLowerCase();
    const signature = await owner.signTypedData(
      eip712.domain as Parameters<Signer['signTypedData']>[0],
      structTypesOf(eip712),
      eip712.message,
    );
    return { address, signature };
  }

  /** Safe-style static multisig blob: 65-byte parts sorted ascending by signer address. */
  async function multisigBlob(eip712: MutableEip712, owners: readonly Signer[]): Promise<string> {
    const parts = await Promise.all(owners.map((owner) => ownerPart(eip712, owner)));
    parts.sort((a, b) => a.address.localeCompare(b.address));
    return `0x${parts.map((p) => p.signature.slice(2)).join('')}`;
  }

  it('parses and decrypts with a 130-byte multisig blob for a wallet userAddress (gate 1 + gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(multisigWalletAddress);
    const signature = await multisigBlob(eip712, [signers.bob, signers.carol]);
    // Two valid owner parts, 130 bytes — opaque per ERC-1271 and forwarded verbatim.
    expect(signature.length).to.equal(2 + 130 * 2);

    const signedPermit = await client.parseSignedDecryptionPermit({
      serializedPermit: {
        version: 2,
        eip712: eip712 as never,
        signature, // 130-byte concatenated blob, forwarded opaquely
        signerAddress: getAddress(multisigWalletAddress),
      },
      transportKeyPair,
    });
    const res = await client.decryptValue({
      contractAddress: multisigWalletAddress as `0x${string}`,
      transportKeyPair,
      signedPermit,
      encryptedValue: (await multisigWallet.value()) as `0x${string}`,
    });
    expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
  });

  it('definitively rejects a single 65-byte owner signature below the wallet threshold (gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(multisigWalletAddress);
    const { signature } = await ownerPart(eip712, signers.bob);
    // Exactly 65 bytes, but a single owner is below the 2-of-3 threshold. The
    // 65-byte EOA fast-path recovers bob (!= the wallet userAddress), so verify
    // falls through to the `isValidSignature` STATICCALL, which returns non-magic
    // (or reverts). The SDK rejects it definitively client-side with an
    // `Erc1271Error` rather than forwarding — the same verdict the relayer's /v3
    // pre-check would return (sync 400).
    expect(signature.length).to.equal(2 + 130);

    const err = await captureRejection(
      client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712: eip712 as never,
          signature,
          signerAddress: getAddress(multisigWalletAddress),
        },
        transportKeyPair,
      }),
      'parseSignedDecryptionPermit with a below-threshold owner signature',
    );
    // Definitive SDK-side ERC-1271 rejection (Erc1271WrongMagicError / Erc1271RejectedError).
    expect(err.message, err.stack).to.match(/erc-1271|isValidSignature|magic|non-magic|reverted/i);
  });

  it('parses and decrypts with the empty approveHash signature (gate 1 + gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(approveWalletAddress);

    // Pre-approve the exact digest on-chain, then decrypt with an empty signature.
    const { EIP712Domain: _d, ...structTypes } = eip712.types;
    const digest = TypedDataEncoder.hash(eip712.domain, structTypes, eip712.message);
    await (await approveWallet.connect(signers.bob).approveHash(digest)).wait();

    const signedPermit = await client.parseSignedDecryptionPermit({
      serializedPermit: {
        version: 2,
        eip712: eip712 as never,
        signature: '0x',
        signerAddress: getAddress(approveWalletAddress),
      },
      transportKeyPair,
    });
    const res = await client.decryptValue({
      contractAddress: approveWalletAddress as `0x${string}`,
      transportKeyPair,
      signedPermit,
      encryptedValue: (await approveWallet.value()) as `0x${string}`,
    });
    expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
  });

  it('hard-wires the signed permit userAddress to the connected signer (signing path stays EOA/self-only)', async function () {
    this.timeout(120_000);
    // `signDecryptionPermit` is deliberately unchanged: it has no parameter for a
    // userAddress distinct from the signer, so the permit it produces always
    // asserts authority over the SIGNER's own handles. A smart-wallet userAddress
    // is therefore inexpressible on the SIGNING path — wallet permits are issued
    // instead via `parseSignedDecryptionPermit` (see the multisig / approveHash
    // cases above), which accepts an externally-assembled blob and an
    // eip712.message.userAddress pointed at the wallet.
    const permit = await client.signDecryptionPermit({
      contractAddresses: [multisigWalletAddress],
      durationSeconds: DURATION_SECONDS,
      startTimestamp,
      transportKeyPair,
      signer: signers.bob,
      signerAddress: signers.bob.address as `0x${string}`,
    });
    expect(getAddress(permit.encryptedDataOwnerAddress)).to.equal(getAddress(signers.bob.address));
    expect(getAddress((permit.eip712.message as { userAddress: string }).userAddress)).to.equal(
      getAddress(signers.bob.address),
    );
  });
});
