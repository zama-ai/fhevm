import type { AbstractEthereumUtils, EncodeCallParameters } from '@fhevm/host-contracts-cleartext/ts';
import {
  encodeAbiParameters as viemEncodeAbiParameters,
  encodeFunctionData,
  getContractAddress as getViemContractAddress,
  getCreate2Address as viemGetCreate2Address,
  keccak256 as viemKeccak256,
  parseAbiParameters,
  toHex,
  type Address,
  type Hex,
} from 'viem';
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

    keccak256(parameters: { readonly bytes: string }): `0x${string}` {
      return viemKeccak256(parameters.bytes as Hex);
    },

    encodeAbiParameters(parameters: {
      readonly types: readonly string[];
      readonly values: readonly unknown[];
    }): `0x${string}` {
      // viem wants parsed parameter descriptors, not type strings; `parseAbiParameters` turns the
      // comma-joined list into them. Standard encoding, which is what `abi.encode` in Solidity is.
      return viemEncodeAbiParameters(parseAbiParameters(parameters.types.join(',')), parameters.values as never);
    },

    getCreate2Address(parameters: {
      readonly from: string;
      readonly salt: string;
      readonly initCodeHash: string;
    }): `0x${string}` {
      return viemGetCreate2Address({
        from: parameters.from as Address,
        salt: parameters.salt as Hex,
        bytecodeHash: parameters.initCodeHash as Hex,
      });
    },
  };
}
