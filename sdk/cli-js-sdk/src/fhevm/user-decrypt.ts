import { createFhevmDecryptClient } from "@fhevm/sdk/viem";
import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type {
  DecryptedValue,
  DecryptionPermitSummary,
  UserDecryptResult,
} from "../types";

type DecryptContext = Pick<
  ClientContext,
  "chain" | "contractAddress" | "publicClient"
>;

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

export const decryptUserValues = async (
  context: DecryptContext,
  options: {
    encryptedValues: readonly Hex[];
    signer: Account;
    ownerAddress: Hex;
    durationDays: number;
    onProgress?: ProgressReporter;
  },
): Promise<UserDecryptResult> => {
  options.onProgress?.("Creating FHEVM decrypt client");
  const client = createFhevmDecryptClient({
    chain: context.chain,
    publicClient: context.publicClient,
  });
  await client.ready;

  options.onProgress?.("Generating transport key pair");
  const transportKeyPair = await client.generateTransportKeyPair();
  const startTimestamp = Math.floor(Date.now() / 1000);

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
        durationDays: options.durationDays,
        startTimestamp,
        signerAddress: options.signer.address,
        signer: options.signer,
      })
    : await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [context.contractAddress],
        durationDays: options.durationDays,
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
  });

  return {
    contractAddress: context.contractAddress,
    ownerAddress: options.ownerAddress,
    signerAddress: options.signer.address,
    isDelegated: signedPermit.isDelegated,
    encryptedValues: options.encryptedValues,
    clearValues: serializeTypedValues(clearValues),
    permit: summarizePermit(signedPermit, {
      contractAddresses: [context.contractAddress],
      durationDays: options.durationDays,
      startTimestamp,
    }),
  };
};

const summarizePermit = (
  permit: {
    isDelegated: boolean;
    signerAddress: string;
    encryptedDataOwnerAddress: string;
    transportPublicKey: string;
    signature: string;
  },
  signedParameters: {
    contractAddresses: readonly string[];
    durationDays: number;
    startTimestamp: number;
  },
): DecryptionPermitSummary => ({
  isDelegated: permit.isDelegated,
  signerAddress: permit.signerAddress as Hex,
  encryptedDataOwnerAddress: permit.encryptedDataOwnerAddress as Hex,
  transportPublicKey: permit.transportPublicKey,
  signature: permit.signature as Hex,
  contractAddresses: signedParameters.contractAddresses,
  startTimestamp: signedParameters.startTimestamp,
  durationDays: signedParameters.durationDays,
});
