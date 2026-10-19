import { type BytesLike, ethers as EthersT } from 'ethers';
import { ProviderError } from 'hardhat/internal/core/providers/errors';

import { HardhatFhevmError } from '../../error';
import { assertHHFhevm } from '../error';

/**
 * Local replacement for `assertIsAddress` from `@fhevm/mock-utils/utils`.
 *
 * Narrows to the `0x${string}` template type, which is what makes values read out of `.env` usable as
 * `CoprocessorConfig` fields without a cast.
 */
export function assertIsAddress(value: unknown, valueName?: string): asserts value is `0x${string}` {
  if (typeof value !== 'string' || !EthersT.isAddress(value)) {
    throw new HardhatFhevmError(`${valueName ?? 'value'} is not a valid Ethereum address. Got '${String(value)}'.`);
  }
}

export async function assertSignersMatchAddresses(signers: EthersT.Signer[], addresses: string[]): Promise<void> {
  assertHHFhevm(Array.isArray(addresses));
  assertHHFhevm(Array.isArray(signers));
  assertHHFhevm(addresses.length === signers.length);

  // Two parallel arrays, so `for...of` alone is not enough — the index is needed to pair them. `entries()`
  // yields the address as `string`, and the signer still has to be pulled out by index and narrowed:
  // `assertHHFhevm` is declared `asserts cond`, so it does that as well as failing.
  for (const [i, expectedAddress] of addresses.entries()) {
    const signer = signers[i];
    assertHHFhevm(signer !== undefined);
    assertHHFhevm(expectedAddress === (await signer.getAddress()));
  }
}

export function extractEVMErrorData(e: unknown): { data: BytesLike; txHash: string } | undefined {
  /*

        If --network localhost
        ======================

        ProviderError.data = {
          message: "Error: VM Exception while processing transac…000bc6c18ca79490f36204c75cc6d6882bb9f335535)",
          txHash: "0x82e9cc197831a924d15e34b8f259dddace04fc0017f33bd28743776e0775ef45",
          data: "0x6475522d000000000000000000000000bc6c18ca79490f36204c75cc6d6882bb9f335535",
        }

        or: 

        If --network hardhat
        ====================

        Error = {
          message: "Error: VM Exception while processing transac…000bc6c18ca79490f36204c75cc6d6882bb9f335535)",
          txHash: "0x82e9cc197831a924d15e34b8f259dddace04fc0017f33bd28743776e0775ef45",
          data: "0x6475522d000000000000000000000000bc6c18ca79490f36204c75cc6d6882bb9f335535",
        }

  */

  let data: BytesLike | undefined;
  let txHash: string | undefined;

  if (ProviderError.isProviderError(e)) {
    // The null test is not redundant: `typeof null` is 'object'.
    if (typeof e.data !== 'object' || e.data === null) {
      return undefined;
    }
    const providerErrorData: object = e.data;

    if ('data' in providerErrorData && EthersT.isBytesLike(providerErrorData.data)) {
      data = providerErrorData.data;
    }
    if ('txHash' in providerErrorData && EthersT.isHexString(providerErrorData.txHash)) {
      txHash = providerErrorData.txHash;
    }
  } else {
    if (e instanceof Error && 'data' in e && EthersT.isBytesLike(e.data)) {
      data = e.data;
    }
    if (e instanceof Error && 'transactionHash' in e && EthersT.isHexString(e.transactionHash)) {
      txHash = e.transactionHash;
    }
  }

  if (data !== undefined) {
    assertHHFhevm(txHash !== undefined);
    return { data, txHash };
  }

  return undefined;
}
