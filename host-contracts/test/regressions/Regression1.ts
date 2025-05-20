import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { Regression1 } from '../types/contracts/Regression1';

describe('Service', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
    this.services = [
      {
        id: ethers.encodeBytes32String('1'),
        serviceType: 'URL',
        serviceEndpoint: 'https://uidai.gov.in/',
      },
      {
        id: ethers.encodeBytes32String('2'),
        serviceType: 'URL',
        serviceEndpoint: 'https://uidai2.gov.in/',
      },
    ];
  });

  beforeEach(async function () {
    const testContract = await deployTestFixture();
    this.testServiceContractAddress = await testContract.getAddress();
    this.testService = testContract;
    this.instances = await createInstances(this.testServiceContractAddress, ethers, this.signers);
  });

  it('should create and update Service', async function () {
    const tAddService = await this.testService.addServices(this.services);
    await tAddService.wait();

    const _services = await this.testService.getServices(this.signers.alice.address);
    expect(_services.length).to.equal(2);

    for (let i = 0; i < this.services.length; i++) {
      expect(_services[i][0]).to.equal(this.services[i].id);
      expect(_services[i][1]).to.equal(this.services[i].serviceType);
      expect(_services[i][2]).to.equal(this.services[i].serviceEndpoint);
    }

    // this now works even on native fhevm, since using the CustomProvider which overrides default estimated gasLimit to 120% of its value
    const tRemoveService = await this.testService.removeService(0);
    // previously on native fhevm it used to work only using this:
    //const tRemoveService = await this.testService.removeService(0, { gasLimit: 500_000 });
    await tRemoveService.wait();

    const _servicesAfter = await this.testService.getServices(this.signers.alice.address);
    expect(_servicesAfter.length).to.equal(1);

    expect(_servicesAfter[0][0]).to.equal(this.services[1].id);
    expect(_servicesAfter[0][1]).to.equal(this.services[1].serviceType);
    expect(_servicesAfter[0][2]).to.equal(this.services[1].serviceEndpoint);
  });
});

async function deployTestFixture(): Promise<Regression1> {
  const signers = await getSigners();

  const testFactory = await ethers.getContractFactory('Regression1');
  const testContract = await testFactory.connect(signers.alice).deploy();
  await testContract.waitForDeployment();
  return testContract;
}
