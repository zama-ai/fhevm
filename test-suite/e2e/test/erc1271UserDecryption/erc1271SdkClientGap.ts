import { expect } from 'chai';
import { TypedDataEncoder, getAddress } from 'ethers';
import type { Signer } from 'ethers';
import { ethers } from 'hardhat';

import { ERC1271ApproveHashWallet, ERC1271MultisigWallet } from '../../types';
import { createInstances } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

// Executable documentation of the `@fhevm/sdk` ERC-1271 client gap.
//
// The protocol backend (relayer /v3 -> gateway -> KMS connector) accepts
// variable-length ERC-1271 signatures — proven end to end by the multisig
// tests in erc1271UserDecryption.ts, which POST the envelope directly. The
// SDK client, however, cannot produce or forward such a request today:
//
//   gate 1 — signature shape: the parse/inject path runtime-asserts the permit
//            signature is exactly 65 bytes (`Bytes65Hex`, core/base/bytes.ts).
//            Concatenated multisig blobs and the empty `0x` (pre-approved-hash
//            flow) signature both throw there. The sign path never accepts an
//            external signature at all — that limitation is gate 3.
//   gate 2 — client-side verification: `verifyKmsUserDecryptEip712V2` is
//            ECDSA-recover-only and requires recovered == signerAddress, with
//            no ERC-1271 branch — so even a well-formed 65-byte single-owner
//            signature for a smart-wallet userAddress is rejected locally.
//   gate 3 — API surface: `signDecryptionPermit` hard-wires the EIP-712
//            `userAddress` to the connected signer; no parameter exists to
//            issue a permit for a contract-wallet userAddress.
//
// Each test below PASSES TODAY by asserting the exact failure. When the SDK
// team ships variable-length ERC-1271 support, these assertions will start
// FAILING — that is the signal to delete the "TODAY" blocks and enable the
// commented "ENABLE WHEN THE SDK SUPPORTS ERC-1271" blocks, which contain the
// intended happy paths. The future API names (`signatureKind`,
// `createDecryptionPermitFromExternalSignature`) are indicative — align them
// with whatever surface the SDK team ships.
//
// The suite skips itself when the legacy `@zama-fhe/relayer-sdk` adapter is
// active (RELAYER_SDK_VERSION set): the gap being pinned is @fhevm/sdk's.

const KNOWN_VALUE = 123456789n;

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
  throw new Error(
    `${label} unexpectedly succeeded — the SDK appears to have gained ERC-1271 support. ` +
      `Enable the commented happy-path blocks in this file and remove the gate assertions.`,
  );
}

describe('ERC-1271 user decryption SDK client gap', function () {
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

    const legitPermit = await client.signDecryptionPermit({
      contractAddresses: [multisigWalletAddress],
      durationSeconds: 7 * 24 * 3600,
      startTimestamp: Math.floor(Date.now() / 1000),
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

  it('test erc1271 sdk gap rejects a multisig concatenated signature at the 65-byte permit gate', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(multisigWalletAddress);
    const signature = await multisigBlob(eip712, [signers.bob, signers.carol]);
    // The whole point: two valid owner parts, 130 bytes — opaque per ERC-1271,
    // and already proven acceptable by the backend in erc1271UserDecryption.ts.
    expect(signature.length).to.equal(2 + 130 * 2);

    // TODAY: the parse/inject path hard-asserts a single 65-byte signature and
    // throws before anything reaches the relayer (gate 1).
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
      'parseSignedDecryptionPermit with a 130-byte multisig blob',
    );
    expect(err.message, err.stack).to.match(/bytes65/i);

    // ENABLE WHEN THE SDK SUPPORTS ERC-1271 (and delete the block above):
    // const signedPermit = await client.parseSignedDecryptionPermit({
    //   serializedPermit: {
    //     version: 2,
    //     eip712: eip712 as never,
    //     signature, // 130-byte concatenated blob, forwarded opaquely
    //     signerAddress: getAddress(multisigWalletAddress),
    //     // indicative — skips client-side ECDSA verify, relayer's ERC-1271
    //     // pre-check (400 on bad blob) is the authority:
    //     // signatureKind: 'erc1271',
    //   },
    //   transportKeyPair,
    // });
    // const res = await client.decryptValue({
    //   contractAddress: multisigWalletAddress as `0x${string}`,
    //   transportKeyPair,
    //   signedPermit,
    //   encryptedValue: (await multisigWallet.value()) as `0x${string}`,
    // });
    // expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
  });

  it('test erc1271 sdk gap rejects a 65-byte owner signature for a wallet userAddress (no ERC-1271 verify branch)', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(multisigWalletAddress);
    const { signature } = await ownerPart(eip712, signers.bob);
    // Exactly 65 bytes — this deliberately PASSES gate 1 to expose gate 2:
    // the client verifies with plain ECDSA recovery and requires
    // recovered == signerAddress. It recovers bob, the signerAddress is the
    // wallet, and there is no isValidSignature fallback.
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
      'parseSignedDecryptionPermit with an owner signature for a wallet userAddress',
    );
    expect(err.message, err.stack).to.match(/not in the list of kms signers/i);

    // ENABLE WHEN THE SDK SUPPORTS ERC-1271 (and delete the block above):
    // NOTE: a single 65-byte part is below the 2-of-3 wallet's threshold, so
    // once the CLIENT stops rejecting it, the RELAYER's ERC-1271 pre-check
    // rejects it instead (sync 400) — this scenario stays a negative, but the
    // rejection moves from the SDK to the protocol, where it belongs:
    // await expectRelayer400(
    //   client.parseSignedDecryptionPermit({ ... signatureKind: 'erc1271' ... })
    //     .then((signedPermit) => client.decryptValue({ ... })),
    // );
  });

  it('test erc1271 sdk gap rejects the empty approveHash signature at the 65-byte permit gate', async function () {
    this.timeout(120_000);
    const eip712 = eip712ForWallet(approveWalletAddress);

    // TODAY: `0x` — the mock's pre-approved-hash flow, explicitly supported by
    // the relayer and KMS connector — cannot pass the client's 65-byte
    // signature assert (gate 1). (Real Safe's empty-signature ERC-1271 path is
    // `signedMessages` with a SafeMessage re-hash; `approveHash` itself is
    // consumed inside checkSignatures via a 65-byte v=1 part.)
    const err = await captureRejection(
      client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712: eip712 as never,
          signature: '0x',
          signerAddress: getAddress(approveWalletAddress),
        },
        transportKeyPair,
      }),
      'parseSignedDecryptionPermit with an empty approveHash signature',
    );
    expect(err.message, err.stack).to.match(/bytes65/i);

    // ENABLE WHEN THE SDK SUPPORTS ERC-1271 (and delete the block above):
    // // Pre-approve the exact digest on-chain, then decrypt with an empty signature.
    // const { EIP712Domain: _d, ...structTypes } = eip712.types;
    // const digest = TypedDataEncoder.hash(eip712.domain, structTypes, eip712.message);
    // await (await approveWallet.connect(signers.bob).approveHash(digest)).wait();
    // const signedPermit = await client.parseSignedDecryptionPermit({
    //   serializedPermit: {
    //     version: 2,
    //     eip712: eip712 as never,
    //     signature: '0x',
    //     signerAddress: getAddress(approveWalletAddress),
    //     // signatureKind: 'erc1271',
    //   },
    //   transportKeyPair,
    // });
    // const res = await client.decryptValue({
    //   contractAddress: approveWalletAddress as `0x${string}`,
    //   transportKeyPair,
    //   signedPermit,
    //   encryptedValue: (await approveWallet.value()) as `0x${string}`,
    // });
    // expect(BigInt(res.value as bigint | number)).to.equal(KNOWN_VALUE);
    void TypedDataEncoder; // imported for the commented happy path above
  });

  it('test erc1271 sdk gap hard-wires the permit userAddress to the connected signer', async function () {
    this.timeout(120_000);
    // TODAY: `signDecryptionPermit` has no parameter for a userAddress
    // distinct from the signer (gate 3) — the permit it produces always
    // asserts authority over the SIGNER's own handles, so a smart-wallet
    // userAddress is inexpressible on the signing path.
    const permit = await client.signDecryptionPermit({
      contractAddresses: [multisigWalletAddress],
      durationSeconds: 7 * 24 * 3600,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer: signers.bob,
      signerAddress: signers.bob.address as `0x${string}`,
    });
    expect(getAddress(permit.encryptedDataOwnerAddress)).to.equal(getAddress(signers.bob.address));
    expect(getAddress((permit.eip712.message as { userAddress: string }).userAddress)).to.equal(
      getAddress(signers.bob.address),
    );

    // ENABLE WHEN THE SDK SUPPORTS ERC-1271 (and delete the block above):
    // // Indicative shape — an entry point that takes an explicit userAddress
    // // and an externally assembled signature blob:
    // const signedPermit = await client.createDecryptionPermitFromExternalSignature({
    //   userAddress: multisigWalletAddress as `0x${string}`,
    //   contractAddresses: [multisigWalletAddress as `0x${string}`],
    //   durationSeconds: 7 * 24 * 3600,
    //   startTimestamp: Math.floor(Date.now() / 1000),
    //   transportKeyPair,
    //   signature: await multisigBlob(eip712ForWallet(multisigWalletAddress), [signers.bob, signers.carol]),
    // });
    // expect(getAddress(signedPermit.encryptedDataOwnerAddress)).to.equal(getAddress(multisigWalletAddress));
  });
});
