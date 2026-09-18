// An ethers-`Interface`-shaped view of one FHEVM contract's custom errors, built on viem. It is what
// `revertedWithCustomErrorArgs` hands to chai's `revertedWithCustomError`, which reads exactly
// `getError(nameOrSelector)` and `decodeErrorResult(fragment, data)` — no ethers needed.

import { type Abi, type Hex, decodeErrorResult, toFunctionSelector } from 'viem';
import { formatAbiItem } from 'viem/utils';

import type { FhevmErrorFragment, FhevmErrorInterface } from '../../types.js';
import type { FhevmContractWrapper } from '../contracts.js';

type AbiError = Extract<Abi[number], { type: 'error' }>;
type Fragment = FhevmErrorFragment & { readonly item: AbiError };

export function createErrorInterface(wrapper: FhevmContractWrapper): FhevmErrorInterface {
  const fragments: Fragment[] = wrapper.abi
    .filter((item): item is AbiError => item.type === 'error')
    .map((item) => ({ item, name: item.name, inputs: item.inputs, selector: toFunctionSelector(formatAbiItem(item)) }));

  const getError = (key: string): Fragment | null =>
    fragments.find((f) => f.name === key || f.selector.toLowerCase() === key.toLowerCase()) ?? null;

  return {
    getError,
    decodeErrorResult(fragment: FhevmErrorFragment, data: Hex) {
      const found = getError(fragment.selector);
      if (found === null) throw new Error(`Unknown custom error '${fragment.name}' for ${wrapper.name}.`);
      const { args } = decodeErrorResult({ abi: [found.item], data });
      const values = [...args];
      // chai's matcher walks the result the ethers way, through `toArray()`.
      return Object.assign(values, { toArray: () => [...values] });
    },
  };
}
