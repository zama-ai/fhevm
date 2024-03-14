import { SignerWithAddress } from '@nomicfoundation/hardhat-ethers/signers';
import chalk from 'chalk';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

import { createInstance } from './utils';

const getSigners = (eSigners: SignerWithAddress[]) => ({
  alice: eSigners[0],
  bob: eSigners[1],
  carol: eSigners[2],
  dave: eSigners[3],
});

task('task:identity:initRegistry')
  .addParam('registry', 'Registry contract address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const contractAddress = taskArguments.registry;

    const signers = await ethers.getSigners();
    const contract = (await ethers.getContractAt('IdentityRegistry', contractAddress)).connect(signers[0]) as any;
    console.log(chalk.bold('Step 1: Adding registrar'));
    console.log(chalk.italic('eg: registry.addRegistrar(wallet, id)'));
    try {
      const addRegistrarTx = await contract.addRegistrar(signers[1], 1);
      await addRegistrarTx.wait();
      console.log("=> Bob is a registrar with registrar id '1'");
    } catch (e) {
      console.log('=> Bob is already a registrar');
    }

    console.log('---');
    console.log(chalk.bold('Step 2: Adding decentralized id for 4 users (Alice, Bob, Carol and Dave)'));
    console.log(chalk.italic('eg: registry.addDid(wallet)'));
    try {
      const tx1 = await contract.connect(signers[1]).addDid(signers[0]);
      const tx2 = await contract.connect(signers[1]).addDid(signers[1]);
      const tx3 = await contract.connect(signers[1]).addDid(signers[2]);
      const tx4 = await contract.connect(signers[1]).addDid(signers[3]);
      await Promise.all([tx1.wait(), tx2.wait(), tx3.wait(), tx4.wait()]);
      console.log('=> Did added for Alice, Bob, Carol and Dave');
    } catch (e) {
      console.log('=> Did was already added');
    }

    console.log('---');

    console.log(chalk.bold('Step 3: Adding country identifiers'));
    console.log(chalk.italic("eg: registry.setIdentifier(wallet, 'country', Enc(1))"));
    const instance = await createInstance(contractAddress, signers[0], ethers);

    const country1 = instance.encrypt64(1);
    const country2 = instance.encrypt64(2);

    const tx1Identifier = await contract.connect(signers[1]).setIdentifier(signers[0], 'country', country1);
    const tx2Identifier = await contract.connect(signers[1]).setIdentifier(signers[1], 'country', country1);
    const tx3Identifier = await contract.connect(signers[1]).setIdentifier(signers[2], 'country', country1);
    const tx4Identifier = await contract.connect(signers[1]).setIdentifier(signers[3], 'country', country2);
    await Promise.all([tx1Identifier.wait(), tx2Identifier.wait(), tx3Identifier.wait(), tx4Identifier.wait()]);
    console.log("=> Alice, Bob and Carol are from country '1'");
    console.log("=> Dave is from country '2'");
  });

task('task:identity:grantAccess')
  .addParam('registry', 'Registry contract address')
  .addParam('erc20', 'ERC20 contract address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const registryAddress = taskArguments.registry;
    const erc20Address = taskArguments.erc20;

    const signers = await ethers.getSigners();
    const registry = (await ethers.getContractAt('IdentityRegistry', registryAddress)) as any;
    const erc20 = await ethers.getContractAt('CompliantERC20', erc20Address);

    console.log(chalk.bold('Step 1: Getting list of identifiers from ERC20 contract'));
    console.log(chalk.italic('eg: erc20.identifiers()'));
    const identifiers = [...(await erc20.identifiers())];
    console.log('=> List of identifiers needed:', identifiers);

    console.log('---');

    console.log(chalk.bold('Step 2: Grant access to the ERC20 contract to these identifiers'));
    console.log(chalk.italic("eg: erc20.grantAccess(erc20Address, ['country', 'blacklist'])"));
    const txs = await Promise.all([
      registry.connect(signers[0]).grantAccess(erc20Address, identifiers),
      registry.connect(signers[1]).grantAccess(erc20Address, identifiers),
      registry.connect(signers[2]).grantAccess(erc20Address, identifiers),
      registry.connect(signers[3]).grantAccess(erc20Address, identifiers),
    ]);
    await Promise.all(txs.map((tx) => tx.wait()));
    console.log('=> Access granted to the ERC20 contract for all users.');
  });

task('task:identity:mint')
  .addParam('erc20', 'ERC20 contract address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const erc20Address = taskArguments.erc20;

    const signers = await ethers.getSigners();
    const erc20 = await ethers.getContractAt('CompliantERC20', erc20Address);

    const instance = await createInstance(erc20Address, signers[0], ethers);

    console.log(chalk.bold('Step 1: Alice mints 100 000 tokens on the compliant contract'));
    console.log(chalk.italic('eg: erc20.mint(Enc(100000))'));
    const transaction = await erc20.mint(100000);
    await transaction.wait();
    console.log('=> 10000 tokens have been minted');

    console.log('---');

    console.log(chalk.bold('Step 2: Alice transfers some tokens'));
    console.log(chalk.italic('eg: erc20.transfer(wallet, Enc(20000))'));
    const amount20k = instance.encrypt64(20000);
    const amount10k = instance.encrypt64(10000);

    const txT1 = await erc20['transfer(address,bytes)'](signers[2], amount20k);
    const txT2 = await erc20['transfer(address,bytes)'](signers[3], amount10k);
    await Promise.all([txT1.wait(), txT2.wait()]);
    console.log('=> Carol received 20000 tokens');
    console.log('=> Dave received 10000 tokens');
  });

task('task:identity:transfer')
  .addParam('erc20', 'ERC20 contract address')
  .addParam('from', 'From wallet')
  .addParam('to', 'To wallet')
  .addParam('amount', 'Amount')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const eSigners = await ethers.getSigners();
    const signers = getSigners(eSigners);
    const erc20Address = taskArguments.erc20;
    const from = taskArguments.from as keyof typeof signers;
    const to = taskArguments.to as keyof typeof signers;
    const amount = taskArguments.amount;

    const erc20 = (await ethers.getContractAt('CompliantERC20', erc20Address)) as any;

    const instance = await createInstance(erc20Address, signers[from], ethers);

    console.log(chalk.bold(`Sending ${amount} from ${from} to ${to}`));
    console.log(chalk.italic(`eg: erc20.transfer(to, Enc(${amount}))`));
    const encryptedAmount = instance.encrypt64(+amount);
    const transaction = await erc20.connect(signers[from])['transfer(address,bytes)'](signers[to], encryptedAmount);
    await transaction.wait();
    console.log(`=> ${amount} tokens have been transferred from ${from} to ${to}`);
  });

task('task:identity:balanceOf')
  .addParam('erc20', 'ERC20 contract address')
  .addParam('user', 'User wallet')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const eSigners = await ethers.getSigners();
    const signers = getSigners(eSigners);
    const erc20Address = taskArguments.erc20;
    const user = taskArguments.user as keyof typeof signers;

    const erc20 = (await ethers.getContractAt('CompliantERC20', erc20Address)) as any;

    const instance = await createInstance(erc20Address, signers[user], ethers);

    const token = instance.getPublicKey(erc20Address)!;

    const balance = await erc20.connect(signers[user]).balanceOf(signers[user], token.publicKey, token.signature);
    console.log(`=> ${chalk.bold('Balance')}: ${instance.decrypt(erc20Address, balance)} tokens`);
  });
