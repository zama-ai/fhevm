import { createTypedValue } from '@fhevm/sdk/base';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import type { EncryptedValue } from '@fhevm/sdk/types';
import { expect } from 'chai';
import { JsonRpcProvider } from 'ethers';
import { ethers } from 'hardhat';
import { network } from 'hardhat';

import {
  aclAddress,
  apiKey,
  gatewayChainID,
  hostChainID,
  inputVerifierAddress,
  kmsVerifierAddress,
  relayerUrl,
  verifyingContractAddressDecryption,
  verifyingContractAddressInputVerification,
} from '../instance';
import { getSigners, initSigners } from '../signers';

const requireRpcUrl = (): string => {
  const url = (network.config as { url?: string }).url;
  if (!url) throw new Error(`network ${network.name} does not expose an RPC URL`);
  return url;
};

const e2eFhevmChain = defineFhevmChain({
  id: hostChainID,
  fhevm: {
    contracts: {
      acl: { address: aclAddress as `0x${string}` },
      inputVerifier: { address: inputVerifierAddress as `0x${string}` },
      kmsVerifier: { address: kmsVerifierAddress as `0x${string}` },
    },
    relayerUrl,
    gateway: {
      id: gatewayChainID,
      contracts: {
        decryption: { address: verifyingContractAddressDecryption as `0x${string}` },
        inputVerification: { address: verifyingContractAddressInputVerification as `0x${string}` },
      },
    },
  },
});

describe('js-sdk e2e', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contractFactory = await ethers.getContractFactory('TestInput');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();

    setFhevmRuntimeConfig(apiKey ? { auth: { type: 'ApiKeyHeader', value: apiKey } } : {});
  });

  it('js-sdk encrypts uint64 input and decrypts computed result', async function () {
    const sdkProvider = new JsonRpcProvider(requireRpcUrl());
    const encryptClient = createFhevmEncryptClient({
      chain: e2eFhevmChain,
      provider: sdkProvider,
    });
    await encryptClient.ready;

    const encryptedInput = await encryptClient.encryptValue({
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
      value: createTypedValue({ type: 'uint64', value: 7n }),
    });

    const tx = await this.contract.add42ToInput64(encryptedInput.encryptedValue, encryptedInput.inputProof);
    const receipt = await tx.wait();
    expect(receipt?.status).to.equal(1);

    const handle = (await this.contract.resUint64()) as EncryptedValue;
    const decryptClient = createFhevmDecryptClient({
      chain: e2eFhevmChain,
      provider: sdkProvider,
    });
    await decryptClient.ready;

    const transportKeypair = await decryptClient.generateTransportKeypair();
    const signedPermit = await decryptClient.signDecryptionPermit({
      transportKeypair,
      contractAddresses: [this.contractAddress],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: this.signers.alice.address,
      signer: this.signers.alice,
    });

    const decryptedValue = await decryptClient.decryptValue({
      encryptedValue: handle,
      contractAddress: this.contractAddress,
      signedPermit,
      transportKeypair,
    });
    expect(decryptedValue.type).to.equal('uint64');
    expect(decryptedValue.value).to.equal(49n);

    const publicValue = await decryptClient.readPublicValue({ encryptedValue: handle });
    expect(publicValue.type).to.equal('uint64');
    expect(publicValue.value).to.equal(49n);
  });
});
