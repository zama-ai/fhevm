import type {
  RelayerDelegatedUserDecryptProgressArgs,
  RelayerUserDecryptProgressArgs,
} from "@fhevm/sdk/actions/decrypt";
import {
  serializeSignedDecryptionPermit,
  serializeTransportKeyPair,
} from "@fhevm/sdk/actions/chain";
import { createFhevmDecryptClient } from "@fhevm/sdk/viem";
import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import type { ClientContext } from "../config";
import {
  describeDecryptedValues,
  type ProgressReporter,
} from "../shared/progress";
import type {
  DecryptedValue,
  DecryptionPermitSummary,
  UserDecryptResult,
  UserDecryptValidationArtifact,
} from "../types";
import type { NetworkName } from "../types";

type DecryptContext = Pick<
  ClientContext,
  "chain" | "contractAddress" | "publicClient"
>;

type UserDecryptProgressArgs =
  | RelayerUserDecryptProgressArgs
  | RelayerDelegatedUserDecryptProgressArgs;

const serializeTypedValues = (
  values: readonly { type: string; value: unknown }[],
): readonly DecryptedValue[] =>
  values.map((value) => ({
    type: value.type,
    value:
      typeof value.value === "bigint"
        ? value.value.toString()
        : String(value.value),
  }));

/**
 * Signs a user-decryption permit and decrypts private handles.
 *
 * When `ownerAddress` differs from the signer, the permit is created as a
 * delegated permit for the encrypted data owner. Callers are responsible for
 * ensuring ACL delegation exists before invoking this adapter.
 */
export const decryptUserValues = async (
  context: DecryptContext,
  options: {
    encryptedValues: readonly Hex[];
    signer: Account;
    ownerAddress: Hex;
    durationSeconds: number;
    network: NetworkName;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  },
): Promise<UserDecryptResult> => {
  if (
    !Number.isSafeInteger(options.durationSeconds) ||
    options.durationSeconds <= 0
  ) {
    throw new RangeError(
      `Permit duration must be a positive safe integer in seconds, received ${options.durationSeconds.toString()}.`,
    );
  }
  options.onProgress?.("Creating FHEVM decrypt client");
  const client = createFhevmDecryptClient({
    chain: context.chain,
    publicClient: context.publicClient,
  });
  await client.ready;

  options.onProgress?.("Generating transport key pair");
  const transportKeyPair = await client.generateTransportKeyPair();
  const startTimestamp = Math.floor(Date.now() / 1000);
  const relayer: { requestId?: string; jobId?: string } = {};
  const relayerOptions = {
    onProgress: (args: UserDecryptProgressArgs) => {
      if ("requestId" in args && typeof args.requestId === "string") {
        relayer.requestId = args.requestId;
      }
      if (typeof args.jobId === "string") {
        relayer.jobId = args.jobId;
      }
    },
  };

  options.onProgress?.(
    options.ownerAddress.toLowerCase() === options.signer.address.toLowerCase()
      ? "Signing user decryption permit"
      : "Signing delegated user decryption permit",
  );
  const isSelfDecrypt =
    options.ownerAddress.toLowerCase() === options.signer.address.toLowerCase();
  const signedPermit = isSelfDecrypt
    ? await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [context.contractAddress],
        durationSeconds: options.durationSeconds,
        startTimestamp,
        signerAddress: options.signer.address,
        signer: options.signer,
      })
    : await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [context.contractAddress],
        durationSeconds: options.durationSeconds,
        startTimestamp,
        signerAddress: options.signer.address,
        signer: options.signer,
        delegatorAddress: options.ownerAddress,
      });

  options.onProgress?.(
    `Requesting user decryption for ${options.encryptedValues.length.toString()} handle(s)`,
  );
  const clearValues = await client.decryptValues({
    encryptedValues: options.encryptedValues,
    contractAddress: context.contractAddress,
    signedPermit,
    transportKeyPair,
    options: relayerOptions,
  });
  const serializedClearValues = serializeTypedValues(clearValues);
  options.onProgress?.(
    `User decryption received ${clearValues.length.toString()} value(s)`,
  );
  options.onProgress?.(
    `User decrypted value(s): ${describeDecryptedValues(serializedClearValues)}`,
  );

  return {
    contractAddress: context.contractAddress,
    ownerAddress: options.ownerAddress,
    signerAddress: options.signer.address,
    isDelegated: signedPermit.isDelegated,
    relayer:
      relayer.requestId || relayer.jobId
        ? { requestId: relayer.requestId, jobId: relayer.jobId }
        : undefined,
    encryptedValues: options.encryptedValues,
    clearValues: serializedClearValues,
    permit: summarizePermit(signedPermit, {
      contractAddresses: [context.contractAddress],
      durationSeconds: options.durationSeconds,
      startTimestamp,
    }),
    validationArtifact: options.includeValidationArtifact
      ? ({
          schemaVersion: 2,
          flow: signedPermit.isDelegated
            ? "delegated-user-decrypt"
            : "user-decrypt",
          network: options.network,
          relayer:
            relayer.requestId || relayer.jobId
              ? { requestId: relayer.requestId, jobId: relayer.jobId }
              : undefined,
          contractAddress: context.contractAddress,
          ownerAddress: options.ownerAddress,
          signerAddress: options.signer.address,
          isDelegated: signedPermit.isDelegated,
          encryptedValues: options.encryptedValues,
          handleContractPairs: options.encryptedValues.map((handle) => ({
            handle,
            contractAddress: context.contractAddress,
          })),
          transportKeyPair: serializeTransportKeyPair(client, {
            transportKeyPair,
          }),
          serializedPermit: serializeSignedDecryptionPermit(client, {
            signedPermit,
          }),
          permit: summarizePermit(signedPermit, {
            contractAddresses: [context.contractAddress],
            durationSeconds: options.durationSeconds,
            startTimestamp,
          }),
        } satisfies UserDecryptValidationArtifact)
      : undefined,
  };
};

const summarizePermit = (
  permit: {
    version: 1 | 2;
    isDelegated: boolean;
    signerAddress: string;
    encryptedDataOwnerAddress: string;
    transportPublicKey: string;
    signature: string;
  },
  signedParameters: {
    contractAddresses: readonly string[];
    durationSeconds: number;
    startTimestamp: number;
  },
): DecryptionPermitSummary => ({
  version: permit.version,
  isDelegated: permit.isDelegated,
  signerAddress: permit.signerAddress as Hex,
  encryptedDataOwnerAddress: permit.encryptedDataOwnerAddress as Hex,
  transportPublicKey: permit.transportPublicKey,
  signature: permit.signature as Hex,
  contractAddresses: signedParameters.contractAddresses,
  startTimestamp: signedParameters.startTimestamp,
  durationSeconds: signedParameters.durationSeconds,
});
