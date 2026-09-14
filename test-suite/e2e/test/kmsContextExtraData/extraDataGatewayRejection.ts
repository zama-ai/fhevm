// Container half of the KMS-context QA case `extradata-gateway-rejection`.
//
// Scenario 3 proved the Relayer refuses a corrupted extraData over HTTP. This one goes a layer
// deeper: bypass the Relayer entirely and submit the same corrupted request as ABI calldata straight
// to the Gateway's `Decryption` contract. The Relayer is a service that can be replaced or
// bypassed; the contract is the boundary that actually has to hold.
//
// ## Which overload, and why it matters
//
// `Decryption.sol` carries TWO `userDecryptionRequest` functions, and they order their checks
// oppositely:
//
//   - legacy `(CtHandleContractPair[], …)` — verifies the EIP-712 signature ON CHAIN (line 506) and
//     only then extracts the context (line 542). A request tampered after signing reverts for the
//     SIGNATURE and never reaches the version check.
//   - unified `(HandleEntry[], …)` — extracts the context as its fourth statement (line 690), before
//     the fee and before the handles are looked at, and never verifies the signature on chain at all
//     (authorization moved to the KMS Connector).
//
// This suite targets the unified overload: it is what the SDK and Relayer use, and it is the one on
// which the scenario's claim is unambiguous. The legacy path is marked `@custom:deprecated`.
//
// ## Where the calldata comes from
//
// The SDK never builds Gateway calldata — it POSTs to the Relayer, and the Relayer submits the
// transaction with its own key. So there is no "SDK calldata" to capture. What there is, and what
// this suite uses, is the SDK's signed unified envelope (captured exactly as in scenario 3): its
// fields map one-to-one onto the unified function's arguments. The request is genuinely the SDK's,
// re-addressed from the Relayer to the Gateway.
//
// ## The control, and why it is deliberately weak
//
// `_collectUserDecryptionFee` transfers a fee token from `msg.sender`, so a fully successful control
// would need the test account funded and approved for it. The corrupted case is unaffected — it
// reverts before the fee — so the control instead asserts the UNCORRUPTED calldata does not revert
// with `UnsupportedExtraDataVersion`. Whether it then fails on the fee or an uncommitted handle is
// irrelevant: what the control has to establish is that the corrupted case's revert is attributable
// to the changed byte and not to the harness having built invalid calldata.
import { expect } from 'chai';
import { Interface, JsonRpcProvider, Wallet, ethers } from 'ethers';
import { ethers as hardhatEthers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstance, createInstances, protocolConfigAddress } from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import { Signers, getSigners, initSigners } from '../signers';
import { captureUnifiedRequest, withVersionByte, type UnifiedEnvelope } from './extraDataRejectionHttp';

const SETUP_TIMEOUT_MS = 3 * 60 * 1000;
const CASE_TIMEOUT_MS = 3 * 60 * 1000;
const DURATION_SECONDS = 10 * 24 * 3600;

/** extraData v2: 1 version byte + 32-byte contextId + 32-byte epochId. */
const EXTRA_DATA_V2_LENGTH = 132;

/** An unsupported version byte: the contract knows only 0x00, 0x01 and 0x02. */
const UNSUPPORTED_VERSION = '03';

/**
 * The unified overload plus the error the scenario expects.
 *
 * Declared here rather than imported from the gateway artifacts: `test-suite/e2e` builds only the
 * host-chain contracts, so the Gateway ABI is not among its generated typechain types.
 */
const DECRYPTION_ABI = [
  'function userDecryptionRequest((bytes32,address,address)[] handles, address userAddress, bytes publicKey, address[] allowedContracts, (uint256,uint256) requestValidity, bytes signature, bytes extraData)',
  'error UnsupportedExtraDataVersion(uint8 version)',
];

const gatewayRpcUrl = () => process.env.GATEWAY_RPC_URL;
const decryptionAddress = () => process.env.DECRYPTION_ADDRESS;
const gatewayKey = () => process.env.GATEWAY_DEPLOYER_PRIVATE_KEY;

const hex32 = (value: bigint) => value.toString(16).padStart(64, '0');

/** Reads an expected id injected by the CLI profile, or undefined when running standalone. */
const expectedFromEnv = (name: string): bigint | undefined => {
  const raw = process.env[name];
  if (raw === undefined || raw.trim() === '') return undefined;
  return BigInt(raw.trim());
};

/**
 * Maps the SDK's unified envelope onto the unified `userDecryptionRequest` arguments.
 *
 * The correspondence is exact — the relayer forwards these same values to this same function — which
 * is what makes the submitted calldata the SDK's request rather than a reconstruction of it.
 */
const envelopeToCallArgs = (envelope: UnifiedEnvelope) => {
  const payload = envelope.attestedPayload as unknown as {
    handles: ReadonlyArray<{ ctHandle: string; contractAddress: string; ownerAddress: string }>;
    userAddress: string;
    publicKey: string;
    allowedContracts: readonly string[];
    requestValidity: { startTimestamp: string; durationSeconds: string };
    extraData: string;
  };
  return [
    payload.handles.map((entry) => [entry.ctHandle, entry.contractAddress, entry.ownerAddress]),
    payload.userAddress,
    payload.publicKey,
    [...payload.allowedContracts],
    [BigInt(payload.requestValidity.startTimestamp), BigInt(payload.requestValidity.durationSeconds)],
    envelope.signature,
    payload.extraData,
  ] as const;
};

describe('KMS context extraData gateway rejection', function () {
  let signers: Signers;
  let contract: UserDecrypt;
  let contractAddress: string;

  let provider: JsonRpcProvider;
  let wallet: Wallet;
  let decryption: string;
  const iface = new Interface(DECRYPTION_ABI);

  /** The active pair, read from the host chain — the values a compliant SDK embeds. */
  let currentContextId: bigint;
  let currentEpochId: bigint;

  before(async function () {
    this.timeout(SETUP_TIMEOUT_MS);
    // The Gateway is a different chain from the one hardhat is pointed at, so it is reached through
    // its own provider. Without these the suite has nothing to submit to and skips rather than
    // reporting a failure that is really a missing configuration.
    if (!gatewayRpcUrl() || !decryptionAddress() || !gatewayKey()) {
      console.log(
        '[kms-context-extradata-gateway] SKIPPED: GATEWAY_RPC_URL / DECRYPTION_ADDRESS / ' +
          'GATEWAY_DEPLOYER_PRIVATE_KEY must all be set to submit calldata directly to the Gateway.',
      );
      this.skip();
    }
    if (!protocolConfigAddress) {
      throw new Error('PROTOCOL_CONFIG_CONTRACT_ADDRESS is required');
    }

    await initSigners(2);
    signers = await getSigners();

    // A guard, not a convenience: the capture below instruments the @fhevm/sdk client, and on a
    // build wired to a different instance the suite would pass without ever exercising it.
    const instances = await createInstances(signers);
    if (!(instances.alice instanceof FhevmSdk)) {
      this.skip();
    }

    provider = new JsonRpcProvider(gatewayRpcUrl());
    wallet = new Wallet(gatewayKey()!, provider);
    decryption = decryptionAddress()!;

    const factory = await hardhatEthers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();

    const protocolConfig = new hardhatEthers.Contract(
      protocolConfigAddress,
      ['function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)'],
      hardhatEthers.provider,
    );
    [currentContextId, currentEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
  });

  /** Captures the envelope the real SDK builds and signs, without letting it reach the Relayer. */
  const captureSdkEnvelope = async function (this: Mocha.Context): Promise<UnifiedEnvelope> {
    const instance = await createInstance();
    if (!(instance instanceof FhevmSdk)) {
      this.skip();
    }
    const client = instance.rawClient;
    const transportKeyPair = await client.generateTransportKeyPair();
    const permit = await client.signUnifiedDecryptionPermit({
      contractAddresses: [contractAddress as `0x${string}`],
      durationSeconds: DURATION_SECONDS,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer: signers.alice,
      signerAddress: signers.alice.address as `0x${string}`,
    });
    const handle = await contract.xUint64();

    return captureUnifiedRequest(() =>
      client.decryptValue({
        contractAddress: contractAddress as `0x${string}`,
        transportKeyPair,
        signedPermit: permit,
        encryptedValue: handle as `0x${string}`,
      }),
    );
  };

  /** Encodes the unified call for an envelope, optionally overriding its extraData. */
  const encodeCall = (envelope: UnifiedEnvelope, extraDataOverride?: string): string => {
    const source = extraDataOverride
      ? { ...envelope, attestedPayload: { ...envelope.attestedPayload, extraData: extraDataOverride } }
      : envelope;
    return iface.encodeFunctionData('userDecryptionRequest', [...envelopeToCallArgs(source)]);
  };

  /** The revert data an `eth_call` of `data` produces, or undefined when the call succeeds. */
  const revertDataOf = async (data: string): Promise<string | undefined> => {
    try {
      await provider.call({ to: decryption, from: wallet.address, data });
      return undefined;
    } catch (error) {
      const revertData = (error as { data?: string; info?: { error?: { data?: string } } }).data;
      return revertData ?? (error as { info?: { error?: { data?: string } } }).info?.error?.data;
    }
  };

  it('test kms context extraData gateway rejection agrees with the orchestrator', async function () {
    const expectedContextId = expectedFromEnv('KMS_QA_EXPECTED_CONTEXT_ID');
    const expectedEpochId = expectedFromEnv('KMS_QA_EXPECTED_EPOCH_ID');
    if (expectedContextId === undefined || expectedEpochId === undefined) {
      this.skip();
    }
    expect(
      currentContextId,
      'the active context moved between the orchestrator read and this suite — a concurrent lifecycle operation',
    ).to.equal(expectedContextId);
    expect(
      currentEpochId,
      'the active epoch moved between the orchestrator read and this suite — a concurrent rotation',
    ).to.equal(expectedEpochId);
  });

  it('test kms context extraData gateway rejection reverts calldata with a corrupted version byte', async function () {
    this.timeout(CASE_TIMEOUT_MS);
    const envelope = await captureSdkEnvelope.call(this);
    const original = envelope.attestedPayload.extraData;

    // The Given, verified rather than assumed: the SDK built a valid v2 envelope for the pair that
    // is active on chain. Without this the corruption below could be mangling something already bad.
    expect(original.length, `the SDK sent a non-v2 extraData: ${original}`).to.equal(EXTRA_DATA_V2_LENGTH);
    expect(original.toLowerCase()).to.equal(`0x02${hex32(currentContextId)}${hex32(currentEpochId)}`.toLowerCase());

    const corrupted = withVersionByte(original, UNSUPPORTED_VERSION);
    expect(corrupted.slice(4), 'only the version byte may change').to.equal(original.slice(4));

    // 1. The precise assertion: WHICH revert. `eth_call` returns the ABI-encoded custom error.
    const revertData = await revertDataOf(encodeCall(envelope, corrupted));
    expect(revertData, 'the Gateway accepted a corrupted extraData').to.be.a('string');
    const decoded = iface.parseError(revertData!);
    expect(decoded?.name, `unexpected revert: ${revertData}`).to.equal('UnsupportedExtraDataVersion');
    expect(Number(decoded!.args[0]), 'the revert must name the version byte that was rejected').to.equal(
      parseInt(UNSUPPORTED_VERSION, 16),
    );

    // 2. The scenario's own wording — "the transaction must revert" — as a real transaction. An
    // explicit gasLimit skips ethers' estimateGas preflight so the transaction is actually mined and
    // reverts on chain, rather than being refused client-side.
    const sent = await wallet.sendTransaction({
      to: decryption,
      data: encodeCall(envelope, corrupted),
      gasLimit: 2_000_000n,
    });
    // ethers v6 THROWS from `wait()` when the receipt reports a revert. Here a revert is the expected
    // outcome, not an error, so the rejection is swallowed and the receipt read back directly.
    await sent.wait().catch(() => undefined);
    const receipt = await provider.getTransactionReceipt(sent.hash);
    expect(receipt, `the corrupted transaction ${sent.hash} was never mined`).to.not.equal(null);
    expect(receipt!.status, `expected the transaction to revert, got status ${receipt!.status}`).to.equal(0);

    // 3. "No decryption request event must be emitted." Stated non-tautologically: look at the block
    // the reverted transaction landed in and require the Decryption contract to have emitted nothing
    // at all. A future change that tolerated the bad version and emitted anyway would fail here.
    expect(receipt!.logs.length, 'a reverted request still produced logs').to.equal(0);
    const blockLogs = await provider.getLogs({
      address: decryption,
      fromBlock: receipt!.blockNumber,
      toBlock: receipt!.blockNumber,
    });
    expect(
      blockLogs.length,
      `the Decryption contract emitted ${blockLogs.length} event(s) in the block of a reverted request`,
    ).to.equal(0);
  });

  it('test kms context extraData gateway rejection does not reject the same calldata when it is untouched', async function () {
    this.timeout(CASE_TIMEOUT_MS);
    // The control. Deliberately weak: a fully successful submission would need this account funded
    // and approved for the user-decryption fee. What matters is only that the untouched calldata is
    // NOT rejected for its extraData version — so the corrupted case's revert is attributable to the
    // byte that was changed, and not to the harness having built calldata the contract would refuse
    // either way.
    const envelope = await captureSdkEnvelope.call(this);
    const revertData = await revertDataOf(encodeCall(envelope));

    if (revertData === undefined || revertData === '0x') {
      return; // Accepted outright: the strongest possible control.
    }
    let decodedName: string | undefined;
    try {
      decodedName = iface.parseError(revertData)?.name;
    } catch {
      decodedName = undefined;
    }
    expect(
      decodedName,
      `the untouched calldata was rejected for its extraData version (${revertData}) — the harness, not the ` +
        `corruption, is what the previous test measured`,
    ).to.not.equal('UnsupportedExtraDataVersion');
    console.log(
      `[kms-context-extradata-gateway] control: untouched calldata reverted with ${revertData.slice(0, 10)} ` +
        `(not UnsupportedExtraDataVersion) — expected on an account that holds no user-decryption fee.`,
    );
  });
});
