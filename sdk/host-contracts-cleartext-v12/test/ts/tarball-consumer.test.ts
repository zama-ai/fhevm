import { deploy, precomputeAddresses } from '@fhevm/host-contracts-cleartext-v12/ts';
import { createPublicClient, http, type Address } from 'viem';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { getContractAddressAtNonce, privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

test('published tarball can be consumed by a viem-backed TypeScript project', async () => {
  expect(typeof deploy).toBe('function');

  const mnemonic = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
  const privateKey = privateKeyFromMnemonic({ mnemonic, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey });

  // v12 CREATE layout for a deployer starting at nonce 0: the 5 core proxies at nonces 1/3/4/5/6,
  // the two cleartext-infra proxies at 7/8, and PauserSet at 9.
  const EXPECTED_FHEVM_ADDRESSES = {
    aclAddress: getContractAddressAtNonce({ privateKey, nonce: 1n }),
    fhevmExecutorAddress: getContractAddressAtNonce({ privateKey, nonce: 3n }),
    kmsVerifierAddress: getContractAddressAtNonce({ privateKey, nonce: 4n }),
    inputVerifierAddress: getContractAddressAtNonce({ privateKey, nonce: 5n }),
    hcuLimitAddress: getContractAddressAtNonce({ privateKey, nonce: 6n }),
  };
  const EXPECTED_CLEARTEXT_ADDRESSES = {
    cleartextArithmeticAddress: getContractAddressAtNonce({ privateKey, nonce: 7n }),
    cleartextDbAddress: getContractAddressAtNonce({ privateKey, nonce: 8n }),
  };
  const EXPECTED_PAUSER_SET_ADDRESS = getContractAddressAtNonce({ privateKey, nonce: 9n });

  const anvil = startAnvil({ port: 8545, mnemonic });
  const { rpcUrl } = anvil;

  try {
    await waitForAnvil(rpcUrl);

    const publicClient = createPublicClient({ chain: foundry, transport: http(rpcUrl) });
    const { provider, signer, utils } = createViemEthereumAdapters({ rpcUrl, privateKey });

    const signerAddress = await signer.getAddress();
    const signerBalance = await publicClient.getBalance({ address: signerAddress as Address });
    expect(signerBalance).toBeGreaterThan(0n);

    const { fhevmAddresses, cleartextAddresses, pauserSetAddress, nextStartNonce } = precomputeAddresses({
      ethUtils: utils,
      from: deployerAddress,
      startNonce: 0n,
    });

    expect(fhevmAddresses).toEqual(EXPECTED_FHEVM_ADDRESSES);
    expect(cleartextAddresses).toEqual(EXPECTED_CLEARTEXT_ADDRESSES);
    expect(pauserSetAddress).toBe(EXPECTED_PAUSER_SET_ADDRESS);
    expect(nextStartNonce).toBe(10n);

    const verifier = {
      verifyingContractSource: deployerAddress,
      chainIDSource: 1n,
      initialSigners: [deployerAddress],
      initialThreshold: 1n,
    };
    const deployed = await deploy({
      ethProvider: provider,
      ethUtils: utils,
      deployer: signer,
      admin: signer,
      precomputed: { fhevmAddresses, cleartextAddresses, pauserSetAddress },
      config: {
        kmsVerifier: verifier,
        inputVerifier: verifier,
        hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
      },
    });

    expect(deployed.fhevmAddresses).toEqual(EXPECTED_FHEVM_ADDRESSES);
    expect(deployed.aclOwnerAddress).not.toBe(pauserSetAddress);
    await expect(publicClient.getChainId()).resolves.toBe(foundry.id);
  } finally {
    await stopAnvil(anvil.process);
  }
});
