import { expect } from 'chai';

import type { CoprocessorConfigStructOutput } from '../../typechain-types/examples/TestEthereumCoprocessorConfig.js';
import type { TestEthereumCoprocessorConfig } from '../../typechain-types/index.js';
import { chainId, ethers } from '../hardhat.js';
import { getSigners, initSigners } from '../signers.js';
import type { Signers } from '../signers.js';

describe('TestEthereumCoprocessorConfig', function () {
  let contract: TestEthereumCoprocessorConfig;
  let signers: Signers;

  before(async function () {
    await initSigners(2);
    signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestEthereumCoprocessorConfig');
    contract = await contractFactory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
  });

  it('test local confidentialProtocolId', async function () {
    const confidentialProtocolId = await contract.connect(signers.carol).confidentialProtocolId();
    // chainid === 31337
    expect(chainId).to.equal(31337);
    expect(confidentialProtocolId).to.equal(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffn);
  });

  it('test local CoprocessorConfig', async function () {
    // chainid === 31337
    expect(chainId).to.equal(31337);
    const cfg: CoprocessorConfigStructOutput = await contract.connect(signers.carol).getCoprocessorConfig();
    // ACL
    expect(cfg[0]).to.eq('0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D');
    expect(cfg.ACLAddress).to.eq('0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D');
    // FHEVMExecutor
    expect(cfg[1]).to.eq('0xe3a9105a3a932253A70F126eb1E3b589C643dD24');
    expect(cfg.CoprocessorAddress).to.eq('0xe3a9105a3a932253A70F126eb1E3b589C643dD24');
    // KMSVerifier
    expect(cfg[2]).to.eq('0x901F8942346f7AB3a01F6D7613119Bca447Bb030');
    expect(cfg.KMSVerifierAddress).to.eq('0x901F8942346f7AB3a01F6D7613119Bca447Bb030');
  });
});
