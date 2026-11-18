// One-time tooling that brings CoprocessorConfig to host chains deployed before it existed, whose ACL
// owner may be the DAO. Delete this file, and the CoprocessorConfig bootstrap step of the host upgrade
// CI, once CoprocessorConfig runs on every live chain. A new host stack gets CoprocessorConfig from
// task:deployAllHostContracts instead.
import { Contract, JsonRpcProvider, type Provider, getAddress } from 'ethers';
import { task, types } from 'hardhat/config';

import {
  buildCoprocessorConfigInitProposal,
  buildCoprocessorContextArgs,
  deployEmptyUUPS,
  ensureAddressesDirectoryExists,
  readExistingHostEnv,
  readHostEnv,
} from './taskDeploy';
import type { Coprocessor } from './utils/coprocessorConfigTasks';
import { getRequiredEnvVar } from './utils/loadVariables';
import { printUpgradeProposal, verifyProposalImplementation } from './utils/upgradeProposal';

// Adds the CoprocessorConfig empty proxy to a host stack deployed before CoprocessorConfig existed.
// task:prepareDeployCoprocessorConfig (DAO) or task:deployCoprocessorConfig (devnet) then initializes it.
task('task:deployEmptyProxyForCoprocessorConfig').setAction(async function (_, { ethers, upgrades, run }) {
  ensureAddressesDirectoryExists();

  if (readExistingHostEnv().COPROCESSOR_CONFIG_CONTRACT_ADDRESS) {
    console.warn(
      'Empty-proxy bootstrap is a no-op; addresses/.env.host already contains COPROCESSOR_CONFIG_CONTRACT_ADDRESS.',
    );
    return;
  }

  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  await run('compile:specific', { contract: 'contracts/emptyProxy' });

  const proxyAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setCoprocessorConfigAddress', { address: proxyAddress });
  process.env.COPROCESSOR_CONFIG_CONTRACT_ADDRESS = proxyAddress;
});

////////////////////////////////////////////////////////////////////////////////
// Check of the first context against the live coprocessor set
////////////////////////////////////////////////////////////////////////////////

// The GatewayConfig and InputVerifier getters as released before CoprocessorConfig.
export const GATEWAY_CONFIG_ABI = [
  'function getCoprocessorTxSenders() view returns (address[])',
  'function getCoprocessor(address coprocessorTxSenderAddress) view returns ((address txSenderAddress, address signerAddress, string s3BucketUrl))',
  'function getCoprocessorMajorityThreshold() view returns (uint256)',
];
const INPUT_VERIFIER_ABI = [
  'function getCoprocessorSigners() view returns (address[])',
  'function getThreshold() view returns (uint256)',
];

export type GatewayCoprocessorSet = {
  coprocessors: { signerAddress: string; storageUrl: string }[];
  majorityThreshold: bigint;
};

export type InputVerifierSignerSet = {
  signers: string[];
  threshold: bigint;
};

export async function readGatewayCoprocessorSet(
  provider: Provider,
  gatewayConfigAddress: string,
  blockTag: number,
): Promise<GatewayCoprocessorSet> {
  const gatewayConfig = new Contract(gatewayConfigAddress, GATEWAY_CONFIG_ABI, provider);
  const txSenders: string[] = await gatewayConfig.getCoprocessorTxSenders({ blockTag });
  const coprocessors = await Promise.all(
    txSenders.map(async (txSender) => {
      const coprocessor = await gatewayConfig.getCoprocessor(txSender, { blockTag });
      return { signerAddress: coprocessor.signerAddress, storageUrl: coprocessor.s3BucketUrl };
    }),
  );
  const majorityThreshold: bigint = await gatewayConfig.getCoprocessorMajorityThreshold({ blockTag });
  return { coprocessors, majorityThreshold };
}

export async function readInputVerifierSignerSet(
  provider: Provider,
  inputVerifierAddress: string,
  blockTag: number,
): Promise<InputVerifierSignerSet> {
  const inputVerifier = new Contract(inputVerifierAddress, INPUT_VERIFIER_ABI, provider);
  const signers: string[] = await inputVerifier.getCoprocessorSigners({ blockTag });
  const threshold: bigint = await inputVerifier.getThreshold({ blockTag });
  return { signers, threshold };
}

// Compares the first context against the live set, matching coprocessors by signer. Returns every
// mismatch. The tx senders and PRF endpoint URLs have no live counterpart, so they are not compared.
export function compareWithLiveCoprocessorSet(
  context: { coprocessors: Coprocessor[]; majorityThreshold: bigint },
  gateway: GatewayCoprocessorSet,
  inputVerifier: InputVerifierSignerSet,
): string[] {
  const mismatches: string[] = [];
  const contextBySigner = new Map(context.coprocessors.map((c) => [getAddress(c.signerAddress), c]));

  if (context.coprocessors.length !== gateway.coprocessors.length) {
    mismatches.push(
      `The context has ${context.coprocessors.length} coprocessors, GatewayConfig has ${gateway.coprocessors.length}.`,
    );
  }
  const gatewaySigners = new Set<string>();
  for (const { signerAddress, storageUrl } of gateway.coprocessors) {
    const signer = getAddress(signerAddress);
    gatewaySigners.add(signer);
    const coprocessor = contextBySigner.get(signer);
    if (!coprocessor) {
      mismatches.push(`GatewayConfig signer ${signer} is missing from the context.`);
    } else if (coprocessor.storageUrl !== storageUrl) {
      mismatches.push(
        `Signer ${signer} has storage URL ${JSON.stringify(coprocessor.storageUrl)} in the context, ${JSON.stringify(storageUrl)} in GatewayConfig.`,
      );
    }
  }
  for (const signer of contextBySigner.keys()) {
    if (!gatewaySigners.has(signer)) {
      mismatches.push(`Context signer ${signer} is not registered in GatewayConfig.`);
    }
  }
  if (context.majorityThreshold !== gateway.majorityThreshold) {
    mismatches.push(
      `The context threshold is ${context.majorityThreshold}, the GatewayConfig majority threshold is ${gateway.majorityThreshold}.`,
    );
  }

  const inputVerifierSigners = new Set(inputVerifier.signers.map((signer) => getAddress(signer)));
  const sameSigners =
    inputVerifierSigners.size === contextBySigner.size &&
    [...contextBySigner.keys()].every((signer) => inputVerifierSigners.has(signer));
  if (!sameSigners) {
    mismatches.push(
      `The context signers [${[...contextBySigner.keys()].join(', ')}] differ from the InputVerifier signers [${[...inputVerifierSigners].join(', ')}].`,
    );
  }
  if (context.majorityThreshold !== inputVerifier.threshold) {
    mismatches.push(
      `The context threshold is ${context.majorityThreshold}, the InputVerifier threshold is ${inputVerifier.threshold}.`,
    );
  }
  return mismatches;
}

// Checks the COPROCESSOR_* env variables against the coprocessor set registered today, in
// GatewayConfig on the Gateway chain and in InputVerifier on this chain. Throws on any mismatch.
export async function assertCoprocessorSetMatchesLiveSet(
  localProvider: Provider,
  options: { gatewayProvider: Provider; gatewayConfigAddress: string },
): Promise<void> {
  const [coprocessors, majorityThreshold] = buildCoprocessorContextArgs();
  const gatewayBlock = await options.gatewayProvider.getBlockNumber();
  const localBlock = await localProvider.getBlockNumber();
  const [gateway, inputVerifier] = await Promise.all([
    readGatewayCoprocessorSet(options.gatewayProvider, options.gatewayConfigAddress, gatewayBlock),
    readInputVerifierSignerSet(localProvider, readHostEnv().INPUT_VERIFIER_CONTRACT_ADDRESS, localBlock),
  ]);

  console.log(
    `Checked the COPROCESSOR_* set against GatewayConfig ${options.gatewayConfigAddress} at Gateway block ${gatewayBlock} and InputVerifier at block ${localBlock}.`,
  );
  const mismatches = compareWithLiveCoprocessorSet(
    { coprocessors, majorityThreshold: BigInt(majorityThreshold) },
    gateway,
    inputVerifier,
  );
  if (mismatches.length > 0) {
    throw new Error(`The COPROCESSOR_* set does not match the live set:\n- ${mismatches.join('\n- ')}`);
  }

  console.log('Signers, storage URLs and threshold match. Review the fields with no live counterpart by hand:');
  for (const coprocessor of coprocessors) {
    console.log(
      `  signer ${coprocessor.signerAddress}: Ethereum tx sender ${coprocessor.txSenderAddress}, PRF endpoint URL ${JSON.stringify(coprocessor.prfEndpointUrl)}`,
    );
  }
}

function addGatewayParams<T extends ReturnType<typeof task>>(definition: T): T {
  return definition
    .addParam('gatewayRpcUrl', 'RPC URL of the Gateway chain', undefined, types.string)
    .addParam('gatewayConfigAddress', 'Address of the GatewayConfig contract', undefined, types.string) as T;
}

addGatewayParams(
  task(
    'task:checkCoprocessorSetAgainstGateway',
    'Checks the COPROCESSOR_* env variables against the coprocessor set in GatewayConfig and in this chain InputVerifier',
  ),
).setAction(async function ({ gatewayRpcUrl, gatewayConfigAddress }, hre) {
  await assertCoprocessorSetMatchesLiveSet(hre.ethers.provider, {
    gatewayProvider: new JsonRpcProvider(gatewayRpcUrl),
    gatewayConfigAddress,
  });
});

////////////////////////////////////////////////////////////////////////////////
// DAO path for the first context
////////////////////////////////////////////////////////////////////////////////

// DAO path for initializing the canonical CoprocessorConfig on a chain whose ACL owner is the DAO. It
// checks the COPROCESSOR_* set against the live set first. Devnet equivalent: task:deployCoprocessorConfig.
addGatewayParams(
  task(
    'task:prepareDeployCoprocessorConfig',
    'Checks the COPROCESSOR_* env variables against the live coprocessor set, then deploys a CoprocessorConfig implementation and prints DAO upgrade calldata that defines the first coprocessor context',
  ),
)
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function ({ gatewayRpcUrl, gatewayConfigAddress, verifyContract }, hre) {
    await assertCoprocessorSetMatchesLiveSet(hre.ethers.provider, {
      gatewayProvider: new JsonRpcProvider(gatewayRpcUrl),
      gatewayConfigAddress,
    });
    const preparedUpgrade = await buildCoprocessorConfigInitProposal(hre);
    printUpgradeProposal(preparedUpgrade);
    if (verifyContract) {
      await verifyProposalImplementation(hre, preparedUpgrade);
    }
    return preparedUpgrade;
  });
