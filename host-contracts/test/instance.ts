import { type ethers as EthersT, hexlify, randomBytes } from 'ethers';
import { network } from 'hardhat';

import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { type MockedDecryptionPermit, createEncryptedInputMocked, userDecryptRequestMocked } from './fhevmjsMocked';
import type { Signers } from './signers';
import { EncryptedInput, FhevmInstance, FhevmInstances, Keypair } from './types';

const verifyingContractAddressDecryption = process.env.DECRYPTION_ADDRESS!;
// Validity of the decryption permits signed by the tests.
const permitDurationDays = 10;

// EIP-712 types of the (non-delegated) user decryption request signed for the KMS.
const userDecryptRequestTypes = {
  UserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
};

const signDecryptionPermit = async (parameters: {
  contractAddress: string;
  signer: EthersT.Signer;
  keypair: Keypair;
}): Promise<MockedDecryptionPermit> => {
  // For user decryption requests, the domain chain is the host chain.
  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: network.config.chainId!,
    verifyingContract: verifyingContractAddressDecryption,
  };
  const message = {
    publicKey: parameters.keypair.publicKey,
    contractAddresses: [parameters.contractAddress],
    startTimestamp: Math.floor(Date.now() / 1000),
    durationDays: permitDurationDays,
    extraData: '0x00',
  };
  const signature = await parameters.signer.signTypedData(domain, userDecryptRequestTypes, message);
  return {
    eip712: { domain, types: userDecryptRequestTypes, primaryType: 'UserDecryptRequestVerification', message },
    signature,
    signerAddress: await parameters.signer.getAddress(),
  };
};

// The mocked KMS never decrypts with the transport key, so random bytes are enough.
const generateKeypair = async (): Promise<Keypair> => ({
  publicKey: hexlify(randomBytes(32)),
  privateKey: hexlify(randomBytes(32)),
});

const createInstanceMocked = async (): Promise<FhevmInstance> => {
  return {
    createEncryptedInput: createEncryptedInputMocked as (
      contractAddress: string,
      userAddress: string,
    ) => EncryptedInput,
    generateKeypair,
    userDecryptSingleHandle: async ({ handle, contractAddress, signer, keypair }) => {
      const permit = await signDecryptionPermit({ contractAddress, signer, keypair });
      return userDecryptRequestMocked(handle, contractAddress, permit);
    },
  };
};

export const createInstances = async (accounts: Signers): Promise<FhevmInstances> => {
  // Encryption and decryption are mocked: the tests only run on the in-process hardhat network.
  if (network.name !== 'hardhat') {
    throw new Error(`Tests are only supported on the hardhat network (current: ${network.name})`);
  }
  const instances: FhevmInstances = {} as FhevmInstances;
  await Promise.all(
    Object.keys(accounts).map(async (k) => {
      instances[k as keyof FhevmInstances] = await createInstanceMocked();
    }),
  );
  return instances;
};

// The debug decryption helpers below read the clear values computed by the mocked coprocessor, so
// they are only available on the hardhat network.
const getMockedClearText = async (handle: string): Promise<string> => {
  if (network.name !== 'hardhat') {
    throw new Error(`Debug decryption is only supported on the hardhat network (current: ${network.name})`);
  }
  await awaitCoprocessor();
  return getClearText(handle);
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bool}
 */
export const decryptBool = async (handle: string): Promise<boolean> => {
  return (await getMockedClearText(handle)) === '1';
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt8 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt16 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt32 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt64 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt128 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {bigint}
 */
export const decrypt256 = async (handle: string): Promise<bigint> => {
  return BigInt(await getMockedClearText(handle));
};

/**
 * @debug
 * This function is intended for debugging purposes only.
 * It reads the clear value from the mocked coprocessor, so it only works on the hardhat network.
 *
 * @param {bigint} handle handle to decrypt
 * @returns {string}
 */
export const decryptAddress = async (handle: string): Promise<string> => {
  const bigintAdd = BigInt(await getMockedClearText(handle));
  const handleStr = '0x' + bigintAdd.toString(16).padStart(40, '0');
  return handleStr;
};
