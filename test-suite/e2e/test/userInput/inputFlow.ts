import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe('Input Flow', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestInput');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    await this.contract.waitForDeployment();
    this.instances = await createInstances(this.signers);
  });

  it('test user input uint64 (non-trivial)', async function () {
    if (!process.env.GATEWAY_NODE_URL) {
      throw new Error('❌ Environment variable GATEWAY_NODE_URL must be set');
    }

    if (!process.env.CIPHERTEXT_COMMITS_CONTRACT_ADDRESS) {
      throw new Error('❌ Environment variable CIPHERTEXT_COMMITS_CONTRACT_ADDRESS must be set');
    }

    if (!process.env.NUMBER_OF_COPROCESSORS) {
      throw new Error('❌ Environment variable NUMBER_OF_COPROCESSORS must be set');
    }
    const GATEWAY_NODE_URL = process.env.GATEWAY_NODE_URL;
    const CIPHERTEXT_COMMITS_CONTRACT_ADDRESS = process.env.CIPHERTEXT_COMMITS_CONTRACT_ADDRESS;
    const NUMBER_OF_COPROCESSORS = parseInt(process.env.NUMBER_OF_COPROCESSORS);

    const CIPHERTEXT_COMMITS_ABI = [
      'function getAddCiphertextMaterialConsensusTxSenders(bytes32 ctHandle) view returns (address[])',
    ];

    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = await inputAlice.encrypt();
    encryptedAmount.handles.forEach((handle: any, index: any) => {
      // Assuming handle is a Uint8Array or Buffer
      console.log(`  Handle ${index}: 0x${Buffer.from(handle).toString('hex')}`);
    });
    console.log('InputProof: 0x' + Buffer.from(encryptedAmount.inputProof).toString('hex'));

    const ctHandleBytes32 = ethers.zeroPadValue(ethers.hexlify(encryptedAmount.handles[0]), 32);

    const tx = await this.contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
    const receipt = await tx.wait();
    expect(receipt.status).to.equal(1);

    // We are checking consensus on ciphertext commits, not on ZKPoK verification.
    // Reason: the ZKPoK verification getter requires a ZKPoK ID, which is harder to obtain here.
    // The ciphertext commits getter, on the other hand, only requires the handle,
    // which we already have at this stage.
    console.log('Fetching consensus senders from Gateway...');
    console.log('ctHandleBytes32:', ctHandleBytes32);
    console.log('GATEWAY_NODE_URL:', GATEWAY_NODE_URL);
    console.log('CIPHERTEXT_COMMITS_CONTRACT_ADDRESS:', CIPHERTEXT_COMMITS_CONTRACT_ADDRESS);
    console.log('CIPHERTEXT_COMMITS_ABI:', CIPHERTEXT_COMMITS_ABI);

    const extProvider = new ethers.JsonRpcProvider(GATEWAY_NODE_URL); // Gateway node
    const commitsExternal = new ethers.Contract(
      CIPHERTEXT_COMMITS_CONTRACT_ADDRESS,
      CIPHERTEXT_COMMITS_ABI,
      extProvider,
    );

    let sleepTime = 15000;
    console.log(`Sleeping ${sleepTime / 1000} seconds before checking consensus for coprocessor senders.`);
    console.log(`This is needed because the add ciphertext is handled asynchronously by coprocessors.`);
    await sleep(sleepTime);

    const senders: string[] = await commitsExternal.getAddCiphertextMaterialConsensusTxSenders(ctHandleBytes32);

    expect(senders, 'should have exactly 3 consensus senders on external chain').to.have.lengthOf(
      NUMBER_OF_COPROCESSORS,
    );

    //  extra safety checks
    const unique = new Set(senders.map((a) => a.toLowerCase()));
    expect(unique.size, 'consensus senders must be unique').to.equal(NUMBER_OF_COPROCESSORS);
    senders.forEach((a, i) =>
      expect(a, `sender[${i}] must not be zero address`).to.not.equal('0x0000000000000000000000000000000000000000'),
    );

    console.log('Gateway chain consensus senders:', senders);
  });
});
