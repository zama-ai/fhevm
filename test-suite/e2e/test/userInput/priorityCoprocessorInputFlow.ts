import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { runAdd42InputAndDecrypt } from './runAdd42InputAndDecrypt';

const GATEWAY_CONFIG_ABI = [
  'function getCoprocessor(address coprocessorTxSenderAddress) view returns (tuple(address txSenderAddress, address signerAddress, string s3BucketUrl))',
  'function getPriorityCoprocessorTxSender() view returns (address)',
  'function setPriorityCoprocessorTxSender(address coprocessorTxSenderAddress)',
  'function removePriorityCoprocessorTxSender()',
];

const GATEWAY_INPUT_VERIFICATION_ABI = [
  'function pause()',
  'function unpause()',
  'function paused() view returns (bool)',
];

const INPUT_VERIFIER_ABI = [
  'function getCoprocessorSigners() view returns (address[])',
  'function getThreshold() view returns (uint256)',
  'function defineNewContext(address[] newSignersSet, uint256 newThreshold)',
];

const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

type PriorityModeSnapshot = {
  gatewayWasPaused: boolean;
  hostSigners: string[];
  hostThreshold: bigint;
  priorityCoprocessorTxSender: string;
};

const requiredEnv = (name: string) => {
  const value = process.env[name];
  if (!value) {
    throw new Error(`${name} is required`);
  }
  return value;
};

const sameAddresses = (left: string[], right: string[]) =>
  left.length === right.length &&
  left.every((address, index) => ethers.getAddress(address) === ethers.getAddress(right[index]));

const getGatewayConfig = () => {
  const provider = new ethers.JsonRpcProvider(requiredEnv('GATEWAY_RPC_URL'));
  const wallet = new ethers.Wallet(requiredEnv('GATEWAY_DEPLOYER_PRIVATE_KEY'), provider);
  return new ethers.Contract(requiredEnv('GATEWAY_CONFIG_ADDRESS'), GATEWAY_CONFIG_ABI, wallet);
};

const getGatewayInputVerification = () => {
  const inputVerificationAddress = requiredEnv('INPUT_VERIFICATION_ADDRESS');
  const provider = new ethers.JsonRpcProvider(requiredEnv('GATEWAY_RPC_URL'));

  return {
    asOwner: new ethers.Contract(
      inputVerificationAddress,
      GATEWAY_INPUT_VERIFICATION_ABI,
      new ethers.Wallet(requiredEnv('GATEWAY_DEPLOYER_PRIVATE_KEY'), provider),
    ),
    asPauser: new ethers.Contract(
      inputVerificationAddress,
      GATEWAY_INPUT_VERIFICATION_ABI,
      new ethers.Wallet(requiredEnv('GATEWAY_PAUSER_PRIVATE_KEY'), provider),
    ),
  };
};

const getInputVerifier = () => {
  const provider = new ethers.JsonRpcProvider(requiredEnv('RPC_URL'));
  const wallet = new ethers.Wallet(requiredEnv('DEPLOYER_PRIVATE_KEY'), provider);
  return new ethers.Contract(requiredEnv('INPUT_VERIFIER_CONTRACT_ADDRESS'), INPUT_VERIFIER_ABI, wallet);
};

const pauseGatewayInputVerification = async (
  gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>,
) => {
  if (!(await gatewayInputVerification.asOwner.paused())) {
    const tx = await gatewayInputVerification.asPauser.pause();
    await tx.wait();
  }
};

const unpauseGatewayInputVerification = async (
  gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>,
) => {
  if (await gatewayInputVerification.asOwner.paused()) {
    const tx = await gatewayInputVerification.asOwner.unpause();
    await tx.wait();
  }
};

const snapshotPriorityMode = async (
  gatewayConfig: ReturnType<typeof getGatewayConfig>,
  gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>,
  inputVerifier: ReturnType<typeof getInputVerifier>,
): Promise<PriorityModeSnapshot> => ({
  gatewayWasPaused: await gatewayInputVerification.asOwner.paused(),
  hostSigners: Array.from(await inputVerifier.getCoprocessorSigners(), (signer: string) => ethers.getAddress(signer)),
  hostThreshold: await inputVerifier.getThreshold(),
  priorityCoprocessorTxSender: await gatewayConfig.getPriorityCoprocessorTxSender(),
});

const configurePriorityMode = async (
  snapshot: PriorityModeSnapshot,
  gatewayConfig: ReturnType<typeof getGatewayConfig>,
  gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>,
  inputVerifier: ReturnType<typeof getInputVerifier>,
) => {
  const priorityCoprocessorTxSender = ethers.getAddress(requiredEnv('PRIORITY_COPROCESSOR_TX_SENDER_ADDRESS'));
  const priorityCoprocessor = await gatewayConfig.getCoprocessor(priorityCoprocessorTxSender);
  const priorityCoprocessorSigner = ethers.getAddress(priorityCoprocessor.signerAddress);

  await pauseGatewayInputVerification(gatewayInputVerification);

  const setPriorityTx = await gatewayConfig.setPriorityCoprocessorTxSender(priorityCoprocessorTxSender);
  await setPriorityTx.wait();
  expect(await gatewayConfig.getPriorityCoprocessorTxSender()).to.equal(priorityCoprocessorTxSender);

  if (snapshot.hostThreshold !== 1n || !sameAddresses(snapshot.hostSigners, [priorityCoprocessorSigner])) {
    const hostTx = await inputVerifier.defineNewContext([priorityCoprocessorSigner], 1);
    await hostTx.wait();
  }

  await unpauseGatewayInputVerification(gatewayInputVerification);
};

const restorePriorityMode = async (
  snapshot: PriorityModeSnapshot,
  gatewayConfig: ReturnType<typeof getGatewayConfig>,
  gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>,
  inputVerifier: ReturnType<typeof getInputVerifier>,
) => {
  await pauseGatewayInputVerification(gatewayInputVerification);

  try {
    const currentHostSigners = Array.from(await inputVerifier.getCoprocessorSigners(), (signer: string) =>
      ethers.getAddress(signer),
    );
    const currentHostThreshold = await inputVerifier.getThreshold();
    if (currentHostThreshold !== snapshot.hostThreshold || !sameAddresses(currentHostSigners, snapshot.hostSigners)) {
      const restoreHostTx = await inputVerifier.defineNewContext(snapshot.hostSigners, snapshot.hostThreshold);
      await restoreHostTx.wait();
    }

    const currentPriority = await gatewayConfig.getPriorityCoprocessorTxSender();
    if (currentPriority.toLowerCase() !== snapshot.priorityCoprocessorTxSender.toLowerCase()) {
      const restorePriorityTx =
        snapshot.priorityCoprocessorTxSender === ZERO_ADDRESS
          ? await gatewayConfig.removePriorityCoprocessorTxSender()
          : await gatewayConfig.setPriorityCoprocessorTxSender(snapshot.priorityCoprocessorTxSender);
      await restorePriorityTx.wait();
    }
  } finally {
    if (!snapshot.gatewayWasPaused) {
      await unpauseGatewayInputVerification(gatewayInputVerification);
    }
  }
};

describe('Priority coprocessor input flow', function () {
  let gatewayConfig: ReturnType<typeof getGatewayConfig>;
  let gatewayInputVerification: ReturnType<typeof getGatewayInputVerification>;
  let inputVerifier: ReturnType<typeof getInputVerifier>;
  let priorityModeSnapshot: PriorityModeSnapshot | undefined;

  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    gatewayConfig = getGatewayConfig();
    gatewayInputVerification = getGatewayInputVerification();
    inputVerifier = getInputVerifier();
  });

  beforeEach(async function () {
    const envContractAddress = process.env.TEST_INPUT_CONTRACT_ADDRESS;
    const contractFactory = await ethers.getContractFactory('TestInput');
    if (!envContractAddress) {
      this.contract = await contractFactory.connect(this.signers.alice).deploy();
      this.contractAddress = await this.contract.getAddress();
      await this.contract.waitForDeployment();
    } else {
      this.contractAddress = envContractAddress;
      this.contract = contractFactory.connect(this.signers.alice).attach(this.contractAddress);
    }
    this.instances = await createInstances(this.signers);

    priorityModeSnapshot = await snapshotPriorityMode(gatewayConfig, gatewayInputVerification, inputVerifier);
    expect(priorityModeSnapshot.gatewayWasPaused).to.equal(
      false,
      'Priority coprocessor input flow requires gateway input verification to start unpaused',
    );
    await configurePriorityMode(priorityModeSnapshot, gatewayConfig, gatewayInputVerification, inputVerifier);
  });

  afterEach(async function () {
    if (priorityModeSnapshot) {
      await restorePriorityMode(priorityModeSnapshot, gatewayConfig, gatewayInputVerification, inputVerifier);
      priorityModeSnapshot = undefined;
    }
  });

  it('test priority coprocessor input flow', async function () {
    await runAdd42InputAndDecrypt.call(this);
  });
});
