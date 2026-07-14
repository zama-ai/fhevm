import type { AbstractEthereumUtils, EncodeCallParameters } from '@fhevm/host-contracts-cleartext-v12/ts';
import { encodeFunctionData, getContractAddress as getViemContractAddress, toHex, type Address } from 'viem';
import { mnemonicToAccount, privateKeyToAccount } from 'viem/accounts';

export type PrivateKeyFromMnemonicArgs = {
  readonly mnemonic: string;
  readonly addressIndex: number;
};

export function privateKeyFromMnemonic(args: PrivateKeyFromMnemonicArgs): `0x${string}` {
  const account = mnemonicToAccount(args.mnemonic, { addressIndex: args.addressIndex });
  const privateKey = account.getHdKey().privateKey;
  if (privateKey === null) {
    throw new Error('Could not derive private key from mnemonic');
  }

  return toHex(privateKey);
}

export function privateKeyToAddress(args: { readonly privateKey: `0x${string}` }): `0x${string}` {
  const account = privateKeyToAccount(args.privateKey);
  return account.address;
}

export function getContractAddressAtNonce(args: {
  readonly nonce: bigint | number;
  readonly privateKey: `0x${string}`;
}): `0x${string}` {
  const account = privateKeyToAccount(args.privateKey);

  return getViemContractAddress({
    from: account.address,
    nonce: BigInt(args.nonce),
  });
}

export function createViemEthereumUtils(): AbstractEthereumUtils {
  return {
    getContractAddress(parameters: { readonly from: string; readonly nonce: bigint }): `0x${string}` {
      return getViemContractAddress({
        from: parameters.from as Address,
        nonce: parameters.nonce,
      });
    },

    encodeCall(parameters: EncodeCallParameters): Promise<`0x${string}`> {
      return Promise.resolve(
        encodeFunctionData({
          abi: parameters.abi,
          functionName: parameters.functionName,
          args: parameters.args,
        }),
      );
    },
  };
}
