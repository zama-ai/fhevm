import { ethers } from 'hardhat';

import { createInstances } from '../test/instance';
import { getSigners, initSigners } from '../test/signers';

const mode = process.env.RFC029_KEY_CONTINUITY_MODE;
if (mode !== 'prepare' && mode !== 'reuse') {
  throw new Error('RFC029_KEY_CONTINUITY_MODE must be prepare or reuse');
}

const main = async () => {
  await initSigners(2);
  const signers = await getSigners();
  const instances = await createInstances(signers);
  const factory = await ethers.getContractFactory('TestInput');

  if (mode === 'prepare') {
    const contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    const address = await contract.getAddress();
    const encrypted = await instances.alice.encryptUint64({
      value: 7n,
      contractAddress: address,
      userAddress: signers.alice.address,
    });
    await (await contract.add42ToInput64(encrypted.handles[0], encrypted.inputProof)).wait();
    const handle = await contract.resUint64();
    const clear = await instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: address,
      signer: signers.alice,
    });
    if (clear !== 49n) throw new Error(`expected 49 before migration, received ${clear}`);
    console.log(`RFC029_KEY_CONTINUITY_CONTRACT=${address}`);
  } else {
    const address = process.env.RFC029_KEY_CONTINUITY_CONTRACT;
    if (!address) throw new Error('RFC029_KEY_CONTINUITY_CONTRACT is required in reuse mode');
    const contract = factory.connect(signers.alice).attach(address);
    await (await contract.add42ToStored64()).wait();
    const handle = await contract.resUint64();
    const clear = await instances.alice.userDecryptSingleHandle({
      handle,
      contractAddress: address,
      signer: signers.alice,
    });
    if (clear !== 91n) throw new Error(`expected 91 after migration, received ${clear}`);
    console.log(`RFC029_KEY_CONTINUITY_REUSED=${handle}`);
  }
};

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
