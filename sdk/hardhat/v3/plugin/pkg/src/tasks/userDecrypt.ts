// `hardhat fhevm user-decrypt <type> <handle> <contract> [--user <index>]`: prints the cleartext of a
// handle the network's account #<index> is allowed to decrypt for <contract>. The account signs the
// decryption permit through the node (`eth_signTypedData_v4`), as a viem wallet client over the
// connection's provider.

import { HardhatPluginError } from 'hardhat/plugins';
import type { NewTaskActionFunction } from 'hardhat/types/tasks';
import { type Address, createWalletClient, custom } from 'viem';

import { PLUGIN_ID } from '../internal/constants.js';
import { isFhevmEuint } from '../internal/fheType.js';
import { FhevmType, type FhevmUser, type HardhatFhevmRuntimeEnvironment } from '../types.js';
import { parseAddressArg, parseHandleArg, parseIndexArg, parseTypeArg } from './args.js';

type Args = { type: string; handle: string; contract: string; user: number };

const userDecryptAction: NewTaskActionFunction<Args> = async ({ type, handle, contract, user }, hre) => {
  const fhevmType = parseTypeArg(type);
  const handleBytes32 = parseHandleArg(handle);
  const contractAddress = parseAddressArg('contract', contract);
  const connection = await hre.network.getOrCreate();

  const accounts = (await connection.provider.request({ method: 'eth_accounts' })) as Address[];
  const account = accounts[parseIndexArg('--user', user, accounts.length)];
  if (account === undefined)
    throw new HardhatPluginError(PLUGIN_ID, `The network provides no account #${String(user)}.`);
  const wallet = createWalletClient({ account, transport: custom(connection.provider) });

  const value = await userDecrypt(connection.fhevm, fhevmType, handleBytes32, contractAddress, wallet);
  console.log(String(value));
  return value;
};

async function userDecrypt(
  fhevm: HardhatFhevmRuntimeEnvironment,
  fhevmType: FhevmType,
  handle: `0x${string}`,
  contract: Address,
  user: FhevmUser,
): Promise<bigint | boolean | string> {
  if (isFhevmEuint(fhevmType)) return fhevm.userDecryptEuint(fhevmType, handle, contract, user);
  if (fhevmType === FhevmType.ebool) return fhevm.userDecryptEbool(handle, contract, user);
  // Every euint and ebool handled above: what is left is eaddress.
  return fhevm.userDecryptEaddress(handle, contract, user);
}

export default userDecryptAction;
