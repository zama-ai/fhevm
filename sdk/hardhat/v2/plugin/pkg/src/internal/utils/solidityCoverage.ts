import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { HardhatFhevmError } from '../../error';
import { logBox } from './log';

export function isSolidityCoverageRunning(hre: HardhatRuntimeEnvironment): boolean {
  //console.log("process.env.SOLIDITY_COVERAGE = " + process.env.SOLIDITY_COVERAGE);
  return '__SOLIDITY_COVERAGE_RUNNING' in hre ? hre.__SOLIDITY_COVERAGE_RUNNING === true : false;
}

function _getHardhatNetworkForSolidityCoverage():
  | {
      allowUnlimitedContractSize: boolean;
      blockGasLimit: number;
      gas: number;
      gasPrice: number;
      initialBaseFeePerGas: number;
    }
  | undefined {
  const nomiclabsUtilsId = 'solidity-coverage/plugins/resources/nomiclabs.utils';
  const apiId = 'solidity-coverage/api';

  try {
    // CommonJS only (ok since HH plugins must be commonjs modules). Typed `unknown` rather than
    // `require`'s `any`: neither path is in solidity-coverage's `exports`, so the checks below are what
    // make using them safe — and `any` would have let them be skipped silently.
    const nomiclabsUtils: unknown = require(nomiclabsUtilsId);
    const apiModule: unknown = require(apiId);

    if (
      typeof nomiclabsUtils !== 'object' ||
      nomiclabsUtils === null ||
      !('configureHardhatEVMGas' in nomiclabsUtils) ||
      typeof nomiclabsUtils.configureHardhatEVMGas !== 'function' ||
      typeof apiModule !== 'function'
    ) {
      return undefined;
    }

    const configureHardhatEVMGas = nomiclabsUtils.configureHardhatEVMGas as (
      network: Record<string, unknown>,
      api: unknown,
    ) => void;
    const SolidityCoverageAPI = apiModule as new (config: object) => unknown;

    const api = new SolidityCoverageAPI({});

    // Mutated in place by `configureHardhatEVMGas` — hence empty here, checked below.
    const hardhatNetworkForCoverage: Record<string, unknown> = {};
    configureHardhatEVMGas(hardhatNetworkForCoverage, api);

    /*
      Make sure we are in sync with solidity-coverage
    */
    if (
      !('allowUnlimitedContractSize' in hardhatNetworkForCoverage) ||
      typeof hardhatNetworkForCoverage.allowUnlimitedContractSize !== 'boolean'
    ) {
      return undefined;
    }
    if (
      !('blockGasLimit' in hardhatNetworkForCoverage) ||
      typeof hardhatNetworkForCoverage.blockGasLimit !== 'number'
    ) {
      return undefined;
    }
    if (!('gas' in hardhatNetworkForCoverage) || typeof hardhatNetworkForCoverage.gas !== 'number') {
      return undefined;
    }
    if (!('gasPrice' in hardhatNetworkForCoverage) || typeof hardhatNetworkForCoverage.gasPrice !== 'number') {
      return undefined;
    }
    if (
      !('initialBaseFeePerGas' in hardhatNetworkForCoverage) ||
      typeof hardhatNetworkForCoverage.initialBaseFeePerGas !== 'number'
    ) {
      return undefined;
    }

    return hardhatNetworkForCoverage as {
      allowUnlimitedContractSize: boolean;
      blockGasLimit: number;
      gas: number;
      gasPrice: number;
      initialBaseFeePerGas: number;
    };
  } catch {
    return undefined;
  }
}

/**
 * Check if not enough gas when using solidity-coverage
 * Fix: `SOLIDITY_COVERAGE=true npx hardhat coverage`
 * or set hre.config.networks.hardhat.blockGasLimit.
 */
export async function checkSolidityCoverageSettings(hre: HardhatRuntimeEnvironment): Promise<void> {
  if (!isSolidityCoverageRunning(hre)) {
    return;
  }

  if ('SOLIDITY_COVERAGE' in process.env && process.env.SOLIDITY_COVERAGE === 'true') {
    return;
  }

  const hardhatNetworkForCoverage = _getHardhatNetworkForSolidityCoverage();

  // Unable to determine the required hardhat network config for solidity coverage.
  // Ask for SOLIDITY_COVERAGE env var only.
  if (!hardhatNetworkForCoverage) {
    const message = `You are trying to run hardhat using solidity coverage without proper setup.
To solve the problem set SOLIDITY_COVERAGE env variable to 'true':

  SOLIDITY_COVERAGE=true npx hardhat coverage`;

    logBox('Wrong Hardhat Network Config for Solidity Coverage.', message, { out: 'stderr' });

    throw new HardhatFhevmError('Wrong hardhat network config for solidity coverage.');
  }

  const blockGasLimit = (await hre.ethers.provider.getBlock('latest'))?.gasLimit;

  let validHardhatNetworkUserConfig: boolean = true;
  let actions = '';

  // Check if all settings have been properly set in the "user" config.
  if (
    hre.userConfig.networks?.hardhat?.allowUnlimitedContractSize !==
    hardhatNetworkForCoverage.allowUnlimitedContractSize
  ) {
    actions += `   - hre.config.networks.hardhat.allowUnlimitedContractSize = ${hardhatNetworkForCoverage.allowUnlimitedContractSize}\n`;
    validHardhatNetworkUserConfig = false;
  }
  if (blockGasLimit !== BigInt(hardhatNetworkForCoverage.blockGasLimit)) {
    actions += `   - hre.config.networks.hardhat.blockGasLimit = 0x${hardhatNetworkForCoverage.blockGasLimit.toString(16)}\n`;
    validHardhatNetworkUserConfig = false;
  }
  if (hre.userConfig.networks?.hardhat?.gas !== hardhatNetworkForCoverage.gas) {
    actions += `   - hre.config.networks.hardhat.gas = 0x${hardhatNetworkForCoverage.gas.toString(16)}\n`;
    validHardhatNetworkUserConfig = false;
  }
  if (hre.userConfig.networks?.hardhat?.gasPrice !== hardhatNetworkForCoverage.gasPrice) {
    actions += `   - hre.config.networks.hardhat.gasPrice = ${hardhatNetworkForCoverage.gasPrice}\n`;
    validHardhatNetworkUserConfig = false;
  }
  if (hre.userConfig.networks?.hardhat?.initialBaseFeePerGas !== hardhatNetworkForCoverage.initialBaseFeePerGas) {
    actions += `   - hre.config.networks.hardhat.initialBaseFeePerGas = ${hardhatNetworkForCoverage.initialBaseFeePerGas}\n`;
    validHardhatNetworkUserConfig = false;
  }

  // If user config is ok. Do nothing.
  if (validHardhatNetworkUserConfig) {
    return;
  }

  // Display a doc message.
  const message = `You are running hardhat using solidity coverage with a wrong hardhat network config.
To solve the problem you can do one of the following:

1. set SOLIDITY_COVERAGE env variable to 'true' (the preferred way)
 ex: 'SOLIDITY_COVERAGE=true npx hardhat coverage'
 
2. or manually set the following hardhat network config parameters: 
${actions}`;

  logBox('Wrong Hardhat Network Config for Solidity Coverage.', message, { out: 'stderr' });

  throw new HardhatFhevmError('Wrong hardhat network config for solidity coverage.');
}
