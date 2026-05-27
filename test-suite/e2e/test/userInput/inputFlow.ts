import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

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

const requiredEnv = (name: string) => {
  const value = process.env[name];
  if (!value) {
    throw new Error(`${name} is required`);
  }
  return value;
};

const getGatewayConfig = () => {
  const gatewayConfigAddress = requiredEnv('GATEWAY_CONFIG_ADDRESS');
  const gatewayRpcUrl = requiredEnv('GATEWAY_RPC_URL');
  const gatewayDeployerPrivateKey = requiredEnv('GATEWAY_DEPLOYER_PRIVATE_KEY');

  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const wallet = new ethers.Wallet(gatewayDeployerPrivateKey, provider);
  return new ethers.Contract(gatewayConfigAddress, GATEWAY_CONFIG_ABI, wallet);
};

const getGatewayInputVerification = () => {
  const inputVerificationAddress = requiredEnv('INPUT_VERIFICATION_ADDRESS');
  const gatewayRpcUrl = requiredEnv('GATEWAY_RPC_URL');
  const gatewayPauserPrivateKey = requiredEnv('GATEWAY_PAUSER_PRIVATE_KEY');
  const gatewayDeployerPrivateKey = requiredEnv('GATEWAY_DEPLOYER_PRIVATE_KEY');

  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  return {
    asOwner: new ethers.Contract(
      inputVerificationAddress,
      GATEWAY_INPUT_VERIFICATION_ABI,
      new ethers.Wallet(gatewayDeployerPrivateKey, provider),
    ),
    asPauser: new ethers.Contract(
      inputVerificationAddress,
      GATEWAY_INPUT_VERIFICATION_ABI,
      new ethers.Wallet(gatewayPauserPrivateKey, provider),
    ),
  };
};

const getInputVerifier = () => {
  const inputVerifierAddress = requiredEnv('INPUT_VERIFIER_CONTRACT_ADDRESS');
  const hostRpcUrl = requiredEnv('RPC_URL');
  const deployerPrivateKey = requiredEnv('DEPLOYER_PRIVATE_KEY');

  const provider = new ethers.JsonRpcProvider(hostRpcUrl);
  const wallet = new ethers.Wallet(deployerPrivateKey, provider);
  return new ethers.Contract(inputVerifierAddress, INPUT_VERIFIER_ABI, wallet);
};

const sameAddresses = (left: string[], right: string[]) =>
  left.length === right.length &&
  left.every((address, index) => ethers.getAddress(address) === ethers.getAddress(right[index]));

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

const runAdd42InputAndDecrypt = async function (this: Mocha.Context) {
  const encryptedInput = await this.instances.alice.encryptUint64({
    value: 7n,
    contractAddress: this.contractAddress,
    userAddress: this.signers.alice.address,
  });

  encryptedInput.handles.forEach((handle: any, index: any) => {
    // Assuming handle is a Uint8Array or Buffer
    console.log(`  Handle ${index}: 0x${Buffer.from(handle).toString('hex')}`);
  });
  console.log('InputProof: 0x' + Buffer.from(encryptedInput.inputProof).toString('hex'));
  const tx = await this.contract.add42ToInput64(encryptedInput.handles[0], encryptedInput.inputProof);
  const receipt = await tx.wait();
  expect(receipt.status).to.equal(1);

  const handle = await this.contract.resUint64();

  // User decrypt the result - should be 7 + 42 = 49.
  const decryptedValue = await this.instances.alice.userDecryptSingleHandle({
    handle,
    contractAddress: this.contractAddress,
    signer: this.signers.alice,
  });
  expect(decryptedValue).to.equal(49n);

  // Public decrypt the result - should be 49.
  const res = await this.instances.alice.publicDecrypt([handle]);
  const expectedRes = {
    [handle]: 49n,
  };
  assert.deepEqual(res.clearValues, expectedRes);
};

describe('Input Flow', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
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
  });

  it('test user input uint64 (non-trivial)', async function () {
    const encryptedAmount = await this.instances.alice.encryptUint64({
      value: 18446744073709550042n,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    encryptedAmount.handles.forEach((handle: any, index: any) => {
      // Assuming handle is a Uint8Array or Buffer
      console.log(`  Handle ${index}: 0x${Buffer.from(handle).toString('hex')}`);
    });
    console.log('InputProof: 0x' + Buffer.from(encryptedAmount.inputProof).toString('hex'));
    const tx = await this.contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    const receipt = await tx.wait();
    expect(receipt.status).to.equal(1);
  });

  it('test add 42 to uint64 input and decrypt', async function () {
    await runAdd42InputAndDecrypt.call(this);
  });

  it('test priority coprocessor input flow', async function () {
    const gatewayConfig = getGatewayConfig();
    const gatewayInputVerification = getGatewayInputVerification();
    const inputVerifier = getInputVerifier();
    const priorityCoprocessorTxSender = ethers.getAddress(requiredEnv('PRIORITY_COPROCESSOR_TX_SENDER_ADDRESS'));

    let originalPriority = ZERO_ADDRESS;
    let originalHostSigners: string[] = [];
    let originalHostThreshold = 0n;
    let hostContextChanged = false;
    originalPriority = await gatewayConfig.getPriorityCoprocessorTxSender();
    originalHostSigners = Array.from(await inputVerifier.getCoprocessorSigners(), (signer) =>
      ethers.getAddress(signer),
    );
    originalHostThreshold = await inputVerifier.getThreshold();
    if (await gatewayInputVerification.asOwner.paused()) {
      throw new Error('Gateway InputVerification must be unpaused before running the priority input flow test');
    }

    try {
      const priorityCoprocessor = await gatewayConfig.getCoprocessor(priorityCoprocessorTxSender);
      const priorityCoprocessorSigner = ethers.getAddress(priorityCoprocessor.signerAddress);

      await pauseGatewayInputVerification(gatewayInputVerification);
      const setTx = await gatewayConfig.setPriorityCoprocessorTxSender(priorityCoprocessorTxSender);
      await setTx.wait();
      expect(await gatewayConfig.getPriorityCoprocessorTxSender()).to.equal(
        ethers.getAddress(priorityCoprocessorTxSender),
      );

      if (originalHostThreshold !== 1n || !sameAddresses(originalHostSigners, [priorityCoprocessorSigner])) {
        const hostTx = await inputVerifier.defineNewContext([priorityCoprocessorSigner], 1);
        await hostTx.wait();
        hostContextChanged = true;
      }
      await unpauseGatewayInputVerification(gatewayInputVerification);

      await runAdd42InputAndDecrypt.call(this);
    } finally {
      try {
        await pauseGatewayInputVerification(gatewayInputVerification);
        if (hostContextChanged) {
          const restoreHostTx = await inputVerifier.defineNewContext(originalHostSigners, originalHostThreshold);
          await restoreHostTx.wait();
        }
      } finally {
        try {
          const currentPriority = await gatewayConfig.getPriorityCoprocessorTxSender();
          if (currentPriority.toLowerCase() !== originalPriority.toLowerCase()) {
            const restoreTx =
              originalPriority === ZERO_ADDRESS
                ? await gatewayConfig.removePriorityCoprocessorTxSender()
                : await gatewayConfig.setPriorityCoprocessorTxSender(originalPriority);
            await restoreTx.wait();
          }
        } finally {
          await unpauseGatewayInputVerification(gatewayInputVerification);
        }
      }
    }
  });
});
