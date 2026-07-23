import type { Account } from "viem/accounts";

/** Minimal SDK surface exercised by the pre-submission preflight. */
export type PreflightSdkClient = Readonly<{
  generateTransportKeyPair: () => Promise<unknown>;
  signLegacyDecryptionPermit: (parameters: Record<string, unknown>) => Promise<unknown>;
}>;

/**
 * Fails fast on SDK/protocol incompatibilities before any load is emitted.
 *
 * Exercises the SDK's pre-submission pipeline — transport-key generation and
 * decryption-permit signing, which is local work plus host-chain protocol
 * reads (`readKmsSignersContext` and friends); nothing reaches the relayer.
 * Both the user-decrypt and public-decrypt journeys perform these same
 * protocol reads before submitting, so an incompatible SDK (e.g. one that
 * requires a newer ProtocolConfig than the deployed environment provides)
 * aborts the run here instead of failing every scheduled request for the
 * full duration.
 */
export const assertSdkPreflight = async (options: {
  flow: string;
  target: string;
  client: PreflightSdkClient;
  contractAddress: string;
  durationSeconds: number;
  signer: Account;
  delegatorAddress?: string;
}): Promise<void> => {
  try {
    const transportKeyPair = await options.client.generateTransportKeyPair();
    await options.client.signLegacyDecryptionPermit({
      transportKeyPair,
      contractAddresses: [options.contractAddress],
      durationSeconds: options.durationSeconds,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: options.signer.address,
      signer: options.signer,
      ...(options.delegatorAddress !== undefined
        ? { delegatorAddress: options.delegatorAddress }
        : {}),
    });
  } catch (error) {
    throw new Error(
      `${options.flow} SDK preflight failed for target ${options.target}; no request was ` +
        `submitted to the relayer and the run was aborted.`,
      { cause: error },
    );
  }
};
