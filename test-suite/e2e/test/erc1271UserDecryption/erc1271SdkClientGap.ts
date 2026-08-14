import { expect } from 'chai';
import { TypedDataEncoder, getAddress } from 'ethers';
import { ethers } from 'hardhat';

import { ERC1271ApproveHashWallet, EncryptedValueHolder } from '../../types';
import { createInstances } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import {
  SafeAccount,
  buildSafeMultisigSignature,
  collectSafeOwnerParts,
  deploySafeAccount,
  resolveSafeInfra,
} from './safe';

// End-to-end coverage of `@fhevm/sdk` ERC-1271 (smart-contract-wallet) support
// on the unified /v3 user-decryption route. The multisig wallet is a REAL Safe
// v1.4.1 proxy (canonical prebuilt artifacts — see ./safe.ts); its owners sign
// the SafeMessage wrap of the permit digest, exactly as the Safe app would.
//
// The protocol backend (relayer /v3 -> gateway -> KMS Connector) accepts
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
//            `isValidSignature` STATICCALL (precautionary; the KMS Connector
//            is authoritative).
//
// A smart-contract-wallet permit is issued by pointing the serialized permit's
// `eip712.message.userAddress` at the wallet and passing the assembled blob to
// `parseSignedDecryptionPermit`. The signing path (`signUnifiedDecryptionPermit`) is
// deliberately unchanged and stays EOA/self-only: it hard-wires `userAddress`
// to the connected signer (asserted below).
//
// Decrypt shape: each wallet's handle lives on a separate EncryptedValueHolder
// contract with `FHE.allow(handle, wallet)`, and the wallet is only the
// userAddress — the realistic setup (a wallet holding confidential tokens) and
// the only one expressible through the SDK: both `checkPersistAllowed`
// (`userAddress != contractAddress`) and the KMS Connector (`userAddress`
// listed in `allowedContracts`) reject a wallet decrypting a handle held by
// itself. That shape stays covered by the raw-envelope protocol suite via
// permissive `allowedContracts: []`.
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

  let safeAccount: SafeAccount;
  let safeAddress: string;
  let approveWallet: ERC1271ApproveHashWallet;
  let approveWalletAddress: string;
  /** One holder per wallet: the dapp-side contract carrying that wallet's handle. */
  let holders: Map<string, { holder: EncryptedValueHolder; holderAddress: string }>;

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

    // A real Safe v1.4.1 2-of-3 proxy (owners bob/carol/dave; canonical
    // prebuilt artifacts) and a Safe-style approveHash mock wallet (owner
    // bob) — the userAddress targets of the tests.
    const infra = await resolveSafeInfra(signers.alice);
    safeAccount = await deploySafeAccount(
      infra,
      signers.alice,
      [signers.bob.address, signers.carol.address, signers.dave.address],
      2,
    );
    safeAddress = safeAccount.address;

    const approveFactory = await ethers.getContractFactory('ERC1271ApproveHashWallet');
    approveWallet = await approveFactory.connect(signers.alice).deploy(signers.bob.address);
    await approveWallet.waitForDeployment();
    approveWalletAddress = await approveWallet.getAddress();

    // One value holder per wallet, granting THAT wallet decrypt access.
    const holderFactory = await ethers.getContractFactory('EncryptedValueHolder');
    holders = new Map();
    for (const walletAddress of [safeAddress, approveWalletAddress]) {
      const holder = await holderFactory.connect(signers.alice).deploy();
      await holder.waitForDeployment();
      const holderAddress = await holder.getAddress();
      await (await holder.connect(signers.alice).initValueFor(KNOWN_VALUE, walletAddress)).wait();
      holders.set(walletAddress, { holder, holderAddress });
    }

    startTimestamp = Math.floor(Date.now() / 1000);
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
  async function eip712ForWallet(walletAddress: string): Promise<MutableEip712> {
    const { holderAddress } = holderOf(walletAddress);
    const selfPermit = await client.signUnifiedDecryptionPermit({
      contractAddresses: [holderAddress as `0x${string}`],
      durationSeconds: DURATION_SECONDS,
      startTimestamp,
      transportKeyPair,
      signer: signers.bob,
      signerAddress: signers.bob.address as `0x${string}`,
    });
    const eip712 = structuredClone(selfPermit.eip712) as unknown as MutableEip712;
    eip712.message.userAddress = getAddress(walletAddress);
    return eip712;
  }

  /** The struct types without EIP712Domain, as ethers' signTypedData expects. */
  function structTypesOf(eip712: MutableEip712): StructTypes {
    const { EIP712Domain: _domain, ...structTypes } = eip712.types;
    return structTypes;
  }

  /**
   * The EIP-712 digest of the permit — what every verifying layer passes to
   * `isValidSignature`. Safe owners do NOT sign this digest directly: the
   * Safe helpers wrap it into the Safe's own SafeMessage typed data first
   * (see ./safe.ts), mirroring what the Safe app does when connected to a
   * dapp requesting an EIP-712 signature.
   */
  function permitDigestOf(eip712: MutableEip712): string {
    return TypedDataEncoder.hash(eip712.domain, structTypesOf(eip712), eip712.message);
  }

  it('parses and decrypts with a 130-byte multisig blob for a wallet userAddress (gate 1 + gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = await eip712ForWallet(safeAddress);
    const signature = await buildSafeMultisigSignature(safeAccount, permitDigestOf(eip712), [
      signers.bob,
      signers.carol,
    ]);
    // Two valid owner parts, 130 bytes — opaque per ERC-1271 and forwarded verbatim.
    expect(signature.length).to.equal(2 + 130 * 2);

    const signedPermit = await client.parseSignedDecryptionPermit({
      serializedPermit: {
        version: 2,
        eip712: eip712 as never,
        signature, // 130-byte concatenated blob, forwarded opaquely
        signerAddress: getAddress(safeAddress),
      },
      transportKeyPair,
    });
    const { holder, holderAddress } = holderOf(safeAddress);
    const res = await client.decryptValue({
      contractAddress: holderAddress as `0x${string}`,
      transportKeyPair,
      signedPermit,
      encryptedValue: (await holder.value()) as `0x${string}`,
    });
    expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
  });

  it('definitively rejects a single 65-byte owner signature below the wallet threshold (gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = await eip712ForWallet(safeAddress);
    const [{ signature }] = await collectSafeOwnerParts(safeAccount, permitDigestOf(eip712), [signers.bob]);
    // Exactly 65 bytes, but a single owner is below the 2-of-3 threshold. The
    // 65-byte EOA fast-path recovers a pseudo-random address (bob signed the
    // SafeMessage wrap, not the permit digest), never the Safe userAddress, so
    // verify falls through to the `isValidSignature` STATICCALL — where the
    // Safe reverts on its length rule (GS020: 65 < threshold * 65). The SDK
    // rejects it definitively client-side with an `Erc1271Error` rather than
    // forwarding — the same verdict the relayer's /v3 pre-check would return
    // (sync 400).
    expect(signature.length).to.equal(2 + 130);

    const err = await captureRejection(
      client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712: eip712 as never,
          signature,
          signerAddress: getAddress(safeAddress),
        },
        transportKeyPair,
      }),
      'parseSignedDecryptionPermit with a below-threshold owner signature',
    );
    // Definitive SDK-side ERC-1271 rejection (the Safe revert surfaces as Erc1271RejectedError).
    expect(err.message, err.stack).to.match(/erc-1271|isValidSignature|magic|non-magic|reverted/i);
  });

  it('parses and decrypts with the empty approveHash signature (gate 1 + gate 2)', async function () {
    this.timeout(120_000);
    const eip712 = await eip712ForWallet(approveWalletAddress);

    // Pre-approve the exact digest on-chain, then decrypt with an empty
    // signature. (This mock approves the raw digest; a real Safe's empty-sig
    // flow goes through `signedMessages`/SignMessageLib instead and its
    // approveHash targets the SafeMessage hash — see the protocol suite.)
    const digest = permitDigestOf(eip712);
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
    const { holder, holderAddress } = holderOf(approveWalletAddress);
    const res = await client.decryptValue({
      contractAddress: holderAddress as `0x${string}`,
      transportKeyPair,
      signedPermit,
      encryptedValue: (await holder.value()) as `0x${string}`,
    });
    expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
  });

  it('hard-wires the signed permit userAddress to the connected signer (signing path stays EOA/self-only)', async function () {
    this.timeout(120_000);
    // `signUnifiedDecryptionPermit` has no parameter for a userAddress distinct from the
    // signer, so the permit it produces always asserts over the signer's own
    // handles (wallet permits go through `parseSignedDecryptionPermit` instead —
    // see the multisig / approveHash cases above).
    const permit = await client.signUnifiedDecryptionPermit({
      contractAddresses: [safeAddress],
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
