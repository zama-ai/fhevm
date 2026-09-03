import { deploy } from '@fhevm/host-contracts-cleartext/ts';
import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';
import setupDebug from 'debug';
import { ethers as EthersT } from 'ethers';
import * as picocolors from 'picocolors';

import { HardhatFhevmError } from '../../error';
import constants from '../constants';
import {
  createEthersEthereumProvider,
  createEthersEthereumSigner,
  createEthersEthereumUtils,
} from '../vendored/ethersEthereumLib';

const debug = setupDebug('@fhevm/hardhat:setup');

const CLEARTEXT = constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE;

/*
  Balances must be very high in case of solidity coverage: code instrumentation considerably
  increases gas cost, so the deployer needs plenty of headroom at startup.
*/
const DEPLOYER_BALANCE = EthersT.parseEther('10000');

/**
 * An `ethers.Provider` that can also issue raw RPC calls. Both `HardhatEthersProvider` and
 * `EthersT.JsonRpcProvider` qualify; only used here for the `*_setBalance` dev-node cheat.
 */
export type FhevmSetupProvider = EthersT.Provider & {
  send(method: string, params: unknown[]): Promise<unknown>;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Stands up the canonical localhost cleartext FHEVM stack.
 *
 * The whole deployment is one call to `@fhevm/host-contracts-cleartext/ts`'s `deploy()`. That
 * function is the source of truth — the package's `FhevmDeploy.sol` and `scripts/anvil-local-v3.sh`
 * are transcriptions of it, not alternatives to it — so the only job left here is to satisfy its
 * three preconditions:
 *
 *   1. `deployer` is account index 5 of the package mnemonic, and it is funded
 *   2. `deployer` is at nonce 0, because every address is `CREATE(deployer, nonce)`
 *   3. `admin` is the same account (`FhevmDeploy._fhevmAdmin()` returns `DEPLOYER_ADDRESS`)
 *
 * `config` is deliberately not passed: the default `DEFAULT_BOOTSTRAP_CONFIG_V13` is what seeds the
 * KMS/coprocessor signer sets that `@fhevm/sdk`'s cleartext relayer holds the keys for. A configured
 * stack is a stack the SDK cannot sign for.
 *
 * `precomputed` *is* passed, although `deploy()` would otherwise derive the same values from the
 * live nonce. Passing the canonical constants turns a wrong start nonce into a loud failure on the
 * very first address instead of a stack that deploys cleanly at the wrong addresses.
 *
 * Idempotent: re-running against an already-deployed node (`hardhat test --network localhost`, where
 * `hardhat node` deployed the stack at startup) is a no-op.
 */
export async function deployFhevmCleartextHostContracts(provider: FhevmSetupProvider): Promise<Deployed> {
  const deployed: Deployed = {
    fhevmAddresses: CLEARTEXT.fhevmAddresses,
    cleartextAddresses: CLEARTEXT.cleartextAddresses,
    pauserSetAddress: CLEARTEXT.pauserSetAddress,
    aclOwnerAddress: EthersT.ZeroAddress,
  };

  if (await __isAlreadyDeployed(provider)) {
    debug(`${picocolors.cyanBright('ACL')} already deployed at ${CLEARTEXT.fhevmAddresses.aclAddress}. Skip deploy.`);
    return {
      ...deployed,
      aclOwnerAddress: await __readACLOwner(provider),
    };
  }

  const deployerWallet = await __resolveDeployer(provider);

  const ethProvider = createEthersEthereumProvider(provider);
  const ethUtils = createEthersEthereumUtils();
  const deployer = createEthersEthereumSigner(deployerWallet);

  debug(`Deploying ${picocolors.cyanBright(CLEARTEXT.name)}@${CLEARTEXT.version} from ${CLEARTEXT.deployerAddress}...`);

  const result = await deploy({
    ethProvider,
    ethUtils,
    deployer,
    // `FhevmDeploy._fhevmAdmin()` returns DEPLOYER_ADDRESS: deployer and admin are one account.
    // Pass the *same adapter object*, not a second one over the same wallet — each adapter keeps its
    // own nonce counter, so two would hand out the same nonce twice.
    admin: deployer,
    precomputed: {
      fhevmAddresses: CLEARTEXT.fhevmAddresses,
      cleartextAddresses: CLEARTEXT.cleartextAddresses,
      pauserSetAddress: CLEARTEXT.pauserSetAddress,
    },
  });

  debug(`${picocolors.cyanBright('ACL')} address                 : ${result.fhevmAddresses.aclAddress}`);
  debug(`${picocolors.cyanBright('FHEVMExecutor')} address       : ${result.fhevmAddresses.fhevmExecutorAddress}`);
  debug(`${picocolors.cyanBright('KMSVerifier')} address         : ${result.fhevmAddresses.kmsVerifierAddress}`);
  debug(`${picocolors.cyanBright('InputVerifier')} address       : ${result.fhevmAddresses.inputVerifierAddress}`);
  debug(`${picocolors.cyanBright('HCULimit')} address            : ${result.fhevmAddresses.hcuLimitAddress}`);
  debug(`${picocolors.cyanBright('ProtocolConfig')} address      : ${result.fhevmAddresses.protocolConfigAddress}`);
  debug(`${picocolors.cyanBright('KMSGeneration')} address       : ${result.fhevmAddresses.kmsGenerationAddress}`);
  debug(
    `${picocolors.cyanBright('CleartextArithmetic')} address : ${result.cleartextAddresses.cleartextArithmeticAddress}`,
  );
  debug(`${picocolors.cyanBright('CleartextDB')} address         : ${result.cleartextAddresses.cleartextDbAddress}`);
  debug(`${picocolors.cyanBright('PauserSet')} address           : ${result.pauserSetAddress}`);
  debug(`${picocolors.cyanBright('ACLOwner')} address            : ${result.aclOwnerAddress}`);

  return result;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The stack is considered present as soon as the ACL proxy carries code. The ACL address is the
 * first one `deploy()` writes to, so a half-deployed node fails later in `assertNoCodeAtTargets`
 * rather than being mistaken for a complete one here.
 */
// eslint-disable-next-line @typescript-eslint/naming-convention
async function __isAlreadyDeployed(provider: FhevmSetupProvider): Promise<boolean> {
  const code = await provider.getCode(CLEARTEXT.fhevmAddresses.aclAddress);
  return code !== '0x';
}

const ACL_OWNER_ABI = [
  {
    type: 'function',
    name: 'owner',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'address' }],
  },
] as const;

// eslint-disable-next-line @typescript-eslint/naming-convention
async function __readACLOwner(provider: FhevmSetupProvider): Promise<string> {
  const acl = new EthersT.Contract(CLEARTEXT.fhevmAddresses.aclAddress, ACL_OWNER_ABI, provider);
  // `getFunction` avoids Contract's index signature, which reads as possibly-undefined.
  const owner: unknown = await acl.getFunction('owner')();
  if (typeof owner !== 'string' || !EthersT.isAddress(owner)) {
    throw new HardhatFhevmError(`ACL at ${CLEARTEXT.fhevmAddresses.aclAddress} returned an invalid owner.`);
  }
  return owner;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The deploying account, derived from the package mnemonic rather than impersonated: the mnemonic is
 * public, so the private key is simply available — the same way `scripts/deploy.sh` gets it with
 * `cast wallet private-key --mnemonic ... --mnemonic-index 5`.
 */
// eslint-disable-next-line @typescript-eslint/naming-convention
async function __resolveDeployer(provider: FhevmSetupProvider): Promise<EthersT.Signer> {
  const wallet = EthersT.HDNodeWallet.fromPhrase(CLEARTEXT.mnemonic, undefined, CLEARTEXT.deployerPath);

  if (wallet.address !== CLEARTEXT.deployerAddress) {
    throw new HardhatFhevmError(
      `Unexpected ${CLEARTEXT.name} deployer address. Derived ${wallet.address} from the package mnemonic at ${CLEARTEXT.deployerPath}, expected ${CLEARTEXT.deployerAddress}.`,
    );
  }

  await __setBalance(provider, wallet.address, DEPLOYER_BALANCE);

  const nonce = await provider.getTransactionCount(wallet.address, 'latest');
  if (nonce !== CLEARTEXT.deployerStartNonce) {
    throw new HardhatFhevmError(
      `The ${CLEARTEXT.name} deployer ${wallet.address} is at nonce ${nonce}, expected ${CLEARTEXT.deployerStartNonce}. ` +
        `Every host contract address is CREATE(deployer, nonce), so the stack can only land on the addresses ` +
        `'@fhevm/solidity/config/ZamaConfig.sol' compiles into your contracts if this account has sent no ` +
        `transaction yet. Restart the node, or make sure nothing else sends from this account.`,
    );
  }

  return wallet.connect(provider);
}

/**
 * Dev-node cheat. anvil aliases the `hardhat_*` namespace, but not every node does, so fall back to
 * the `anvil_*` spelling before giving up.
 */
// eslint-disable-next-line @typescript-eslint/naming-convention
async function __setBalance(provider: FhevmSetupProvider, address: string, balance: bigint): Promise<void> {
  const params = [address, EthersT.toQuantity(balance)];
  try {
    await provider.send('hardhat_setBalance', params);
  } catch {
    try {
      await provider.send('anvil_setBalance', params);
    } catch {
      throw new HardhatFhevmError(
        `Unable to fund the ${CLEARTEXT.name} deployer ${address}: the network supports neither 'hardhat_setBalance' nor 'anvil_setBalance'.`,
      );
    }
  }
}
