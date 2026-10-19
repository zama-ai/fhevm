import { scope } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { HardhatFhevmError } from '../error';
import { fhevmContext } from '../internal/EnvironmentExtender';
import { type CoprocessorConfig, getCoprocessorConfig } from '../internal/coprocessorConfig';
import type { FhevmType } from '../internal/fheType';
import { isFhevmEaddress, isFhevmEbool, isFhevmEuint, tryParseFhevmType } from '../internal/fheType';
import {
  SCOPE_FHEVM,
  SCOPE_FHEVM_TASK_CHECK_FHEVM_COMPATIBILITY,
  SCOPE_FHEVM_TASK_PUBLIC_DECRYPT,
  SCOPE_FHEVM_TASK_RESOLVE_FHEVM_CONFIG,
  SCOPE_FHEVM_TASK_USER_DECRYPT,
} from '../task-names';

import picocolors = require('picocolors');

const fhevmScope = scope(SCOPE_FHEVM, 'Fhevm related commands');

fhevmScope
  .task(SCOPE_FHEVM_TASK_USER_DECRYPT)
  .setDescription('Performs a user decryption of the specified byte-32 handle')
  .addParam('type', 'Specify the FHEVM primitive type name (e.g. ebool, euint8, euint16, etc.)')
  .addParam('handle', 'Specify the byte-32 handle to decrypt')
  .addParam('user', 'Specify which user account index')
  .addParam('contract', 'Specify the contract address')
  .setAction(
    async (
      {
        type,
        handle,
        user,
        contract,
      }: {
        type: string;
        handle: string;
        user: string;
        contract: string;
      },
      hre: HardhatRuntimeEnvironment,
    ) => {
      const fhevmEnv = fhevmContext.get();
      await fhevmEnv.initializeCLIApi();

      const t: FhevmType | undefined = tryParseFhevmType(type);
      if (t === undefined) {
        throw new HardhatFhevmError(`Unknown FHEVM primitive type name ${type}`);
      }

      const accountIndex: number = Number.parseInt(user);
      if (Number.isNaN(accountIndex) || !Number.isInteger(accountIndex) || accountIndex < 0) {
        throw new HardhatFhevmError(`Invalid account index '${user}', expecting a positive integer.`);
      }

      const signers = await hre.ethers.getSigners();
      const signer = signers[accountIndex];
      if (signer === undefined) {
        throw new HardhatFhevmError(
          `Invalid account index '${user}', expecting a positive integer between 0 and ${signers.length - 1}.`,
        );
      }
      //npx hardhat fhevm user-decrypt --type euint32 --handle 0x9b01877d34a170d07905d4a4224a6ca6f7bc1f5695ff0000000000aa36a70400 --user 0 --contract 0x2C6A7e015B77E7c984c0a9280b89aa70721DEeCe
      if (isFhevmEuint(t)) {
        try {
          const clearUint = await hre.fhevm.userDecryptEuint(t, handle, contract, signer);
          console.log(clearUint);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else if (isFhevmEbool(t)) {
        try {
          const clearBool = await hre.fhevm.userDecryptEbool(handle, contract, signer);
          console.log(clearBool);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else if (isFhevmEaddress(t)) {
        try {
          const clearAddress = await hre.fhevm.userDecryptEaddress(handle, contract, signer);
          console.log(clearAddress);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else {
        throw new HardhatFhevmError(`Unsupported FHEVM type: ${t}`);
      }
    },
  );

fhevmScope
  .task(SCOPE_FHEVM_TASK_PUBLIC_DECRYPT)
  .setDescription('Performs a public decryption of the specified byte-32 handle')
  .addParam('type', 'Specify the FHEVM primitive type name (e.g. ebool, euint8, euint16, etc.)')
  .addParam('handle', 'Specify the byte-32 handle to decrypt')
  .setAction(
    async (
      {
        type,
        handle,
      }: {
        type: string;
        handle: string;
      },
      hre: HardhatRuntimeEnvironment,
    ) => {
      const fhevmEnv = fhevmContext.get();
      await fhevmEnv.initializeCLIApi();

      const t: FhevmType | undefined = tryParseFhevmType(type);
      if (t === undefined) {
        throw new HardhatFhevmError(`Unknown FHEVM primitive type name ${type}`);
      }

      if (isFhevmEuint(t)) {
        try {
          const clearUint = await hre.fhevm.publicDecryptEuint(t, handle);
          console.log(clearUint);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else if (isFhevmEbool(t)) {
        try {
          const clearBool = await hre.fhevm.publicDecryptEbool(handle);
          console.log(clearBool);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else if (isFhevmEaddress(t)) {
        try {
          const clearAddress = await hre.fhevm.publicDecryptEaddress(handle);
          console.log(clearAddress);
        } catch (e) {
          if (e instanceof Error) {
            throw new HardhatFhevmError(e.message, e);
          } else {
            throw e;
          }
        }
      } else {
        throw new HardhatFhevmError(`Unsupported FHEVM type: ${t}`);
      }
    },
  );

// The command below will check if `0x8D94d6f1593A50DDF52D317016e3dD1af1EE1292` is correctly
// configured for FHEVM Sepolia:
// npx hardhat --network sepolia fhevm check-fhevm-compatibility --address 0x8D94d6f1593A50DDF52D317016e3dD1af1EE1292
fhevmScope
  .task(SCOPE_FHEVM_TASK_CHECK_FHEVM_COMPATIBILITY)
  .setDescription('Checks if a FHEVM contract is well configured to perform FHEVM operations')
  .addParam('address', 'Specify the contract address')
  .setAction(
    async (
      {
        address,
      }: {
        address: string;
      },
      hre: HardhatRuntimeEnvironment,
    ) => {
      if (!hre.ethers.isAddress(address)) {
        throw new HardhatFhevmError(`Invalid --address parameter value. '${address}' is not a valid address.`);
      }
      const fhevmEnv = fhevmContext.get();
      await fhevmEnv.minimalInitWithAddresses();

      const coprocessorConfig = await getCoprocessorConfig(hre.ethers.provider, address);
      if (
        coprocessorConfig.ACLAddress === hre.ethers.ZeroAddress &&
        coprocessorConfig.CoprocessorAddress === hre.ethers.ZeroAddress &&
        coprocessorConfig.KMSVerifierAddress === hre.ethers.ZeroAddress
      ) {
        const deployedCode = await fhevmEnv.cleartextProvider.getCodeAt(address);
        if ((deployedCode as unknown) === undefined || deployedCode === '0x') {
          throw new HardhatFhevmError(`The address '${address}' does not correspond to a deployed contract.`);
        }
      }

      const expected: CoprocessorConfig = {
        ACLAddress: fhevmEnv.getACLAddress(),
        CoprocessorAddress: fhevmEnv.getFHEVMExecutorAddress(),
        KMSVerifierAddress: fhevmEnv.getKMSVerifierAddress(),
      };

      if (
        coprocessorConfig.ACLAddress !== expected.ACLAddress ||
        coprocessorConfig.CoprocessorAddress !== expected.CoprocessorAddress ||
        coprocessorConfig.KMSVerifierAddress !== expected.KMSVerifierAddress
      ) {
        console.log(
          picocolors.red(
            `The contract deployed at ${address} is configured with an invalid FHEVM Coprocessor Configuration.`,
          ),
        );
        console.log(picocolors.red("The contract's configuration is:"));
        console.log(picocolors.red(JSON.stringify(coprocessorConfig, null, 2)));
        console.log(picocolors.red('The expected configuration is:'));
        console.log(picocolors.red(JSON.stringify(expected, null, 2)));
        throw new HardhatFhevmError(
          `The contract deployed at ${address} is not using a valid Coprocessor configuration`,
        );
      }

      console.log(
        picocolors.green(
          `The contract deployed at ${address} is configured with the valid FHEVM Coprocessor Configuration:`,
        ),
      );
      console.log(JSON.stringify(coprocessorConfig, null, 2));
    },
  );

// npx hardhat --network sepolia fhevm resolve-fhevm-config --acl 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D --kms 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A
fhevmScope
  .task(SCOPE_FHEVM_TASK_RESOLVE_FHEVM_CONFIG)
  .setDescription('Resolve full FHEVM configuration')
  .addParam('acl', 'Specify the acl contract address')
  .addParam('kms', 'Specify the kms contract address')
  .setAction(
    async (
      {
        acl: _acl,
        kms: _kms,
      }: {
        acl: string;
        kms: string;
      },
      _hre: HardhatRuntimeEnvironment,
      // eslint-disable-next-line @typescript-eslint/require-await
    ) => {
      /*
        TODO(migration step 4): rebuild on `@fhevm/sdk`'s `resolveFhevmConfig`, which does exactly
        this. It takes a `Fhevm` client, and `createFhevmBaseClient` needs a complete `FhevmChain` —
        whereas this command is given only an ACL and a KMSVerifier address and is meant to discover
        the rest. Resolving that shape is what makes it more than an import swap.
      */
      throw new HardhatFhevmError(
        `'hardhat ${SCOPE_FHEVM} ${SCOPE_FHEVM_TASK_RESOLVE_FHEVM_CONFIG}' is not implemented yet: the FHEVM hardhat plugin is being migrated to @fhevm/sdk. See plans/MIGRATION_TO_FHEVM_SDK_CLEARTEXT.md.`,
      );
    },
  );
