import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

const GATEWAY_CONFIG_ABI = [
  'function getPriorityCoprocessorTxSender() view returns (address)',
  'function setPriorityCoprocessorTxSender(address coprocessorTxSenderAddress)',
  'function removePriorityCoprocessorTxSender()',
];

const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

const getGatewayConfig = () => {
  const gatewayConfigAddress = process.env.GATEWAY_CONFIG_ADDRESS;
  const gatewayRpcUrl = process.env.GATEWAY_RPC_URL;
  const gatewayDeployerPrivateKey = process.env.GATEWAY_DEPLOYER_PRIVATE_KEY;

  if (!gatewayConfigAddress || !gatewayRpcUrl || !gatewayDeployerPrivateKey) {
    return undefined;
  }

  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const wallet = new ethers.Wallet(gatewayDeployerPrivateKey, provider);
  return new ethers.Contract(gatewayConfigAddress, GATEWAY_CONFIG_ABI, wallet);
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
    const priorityCoprocessorTxSenderEnv = process.env.PRIORITY_COPROCESSOR_TX_SENDER_ADDRESS;

    if (!gatewayConfig || !priorityCoprocessorTxSenderEnv) {
      this.skip();
      return;
    }
    const priorityCoprocessorTxSender = ethers.getAddress(priorityCoprocessorTxSenderEnv);

    let originalPriority = ZERO_ADDRESS;
    try {
      originalPriority = await gatewayConfig.getPriorityCoprocessorTxSender();
    } catch {
      this.skip();
      return;
    }

    try {
      const setTx = await gatewayConfig.setPriorityCoprocessorTxSender(priorityCoprocessorTxSender);
      await setTx.wait();
      expect(await gatewayConfig.getPriorityCoprocessorTxSender()).to.equal(
        ethers.getAddress(priorityCoprocessorTxSender),
      );

      await runAdd42InputAndDecrypt.call(this);
    } finally {
      const currentPriority = await gatewayConfig.getPriorityCoprocessorTxSender();
      if (currentPriority.toLowerCase() !== originalPriority.toLowerCase()) {
        const restoreTx =
          originalPriority === ZERO_ADDRESS
            ? await gatewayConfig.removePriorityCoprocessorTxSender()
            : await gatewayConfig.setPriorityCoprocessorTxSender(originalPriority);
        await restoreTx.wait();
      }
    }
  });
});
