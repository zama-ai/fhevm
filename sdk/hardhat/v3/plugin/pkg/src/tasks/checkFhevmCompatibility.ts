// `hardhat fhevm check-fhevm-compatibility <address>`: is the contract at <address> configured for the
// FHEVM stack this network runs? Distinguishes "no contract there" from "deployed but not initialized"
// from "initialized against another stack", then prints the configuration it found.

import { HardhatPluginError } from 'hardhat/plugins';
import type { NewTaskActionFunction } from 'hardhat/types/tasks';

import { PLUGIN_ID } from '../internal/constants.js';
import type { CoprocessorConfig } from '../types.js';
import { parseAddressArg } from './args.js';

type Args = { address: string };

const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

const checkFhevmCompatibilityAction: NewTaskActionFunction<Args> = async ({ address }, hre) => {
  const contractAddress = parseAddressArg('address', address);
  const connection = await hre.network.getOrCreate();

  const config = await connection.fhevm.getCoprocessorConfig(contractAddress);
  if (Object.values(config).every((value) => value === ZERO_ADDRESS)) {
    const code: unknown = await connection.provider.request({
      method: 'eth_getCode',
      params: [contractAddress, 'latest'],
    });
    if (typeof code !== 'string' || code === '0x') {
      throw new HardhatPluginError(
        PLUGIN_ID,
        `The address '${contractAddress}' does not correspond to a deployed contract.`,
      );
    }
  }

  try {
    await connection.fhevm.assertCoprocessorInitialized(contractAddress);
  } catch (e) {
    console.error(
      `The contract deployed at ${contractAddress} is configured with an invalid FHEVM Coprocessor Configuration.`,
    );
    console.error("The contract's configuration is:");
    console.error(format(config));
    throw e;
  }

  console.log(
    `The contract deployed at ${contractAddress} is configured with the valid FHEVM Coprocessor Configuration:`,
  );
  console.log(format(config));
  return config;
};

function format(config: CoprocessorConfig): string {
  return JSON.stringify(config, null, 2);
}

export default checkFhevmCompatibilityAction;
