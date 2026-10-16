import { getContractAddress as getViemContractAddress, toHex } from 'viem';
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
