import { readFileSync } from 'node:fs';
import { join } from 'node:path';

// Single source of truth for FHETest addresses lives in the sibling JSON
// file so the shell deploy scripts (`assert_fhetest_address_in_abi_v2` in
// fhevm-lib.sh) can also read/validate them.
export const FHETestAddresses = JSON.parse(
  readFileSync(join(import.meta.dirname, 'fhe-test-addresses-v2.json'), 'utf-8'),
) as Readonly<{
  localhost: string;
  localstack: string;
  devnet: string;
  sepolia: string;
}>;

export const FHETestABI = [
  {
    type: 'function',
    name: 'CONTRACT_NAME',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'string',
        internalType: 'string',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'addEuint128',
    inputs: [
      {
        name: 'inputEuint128',
        type: 'bytes32',
        internalType: 'externalEuint128',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint128',
        internalType: 'uint128',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'addEuint16',
    inputs: [
      {
        name: 'inputEuint16',
        type: 'bytes32',
        internalType: 'externalEuint16',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint16',
        internalType: 'uint16',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'addEuint32',
    inputs: [
      {
        name: 'inputEuint32',
        type: 'bytes32',
        internalType: 'externalEuint32',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint32',
        internalType: 'uint32',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'addEuint64',
    inputs: [
      {
        name: 'inputEuint64',
        type: 'bytes32',
        internalType: 'externalEuint64',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint64',
        internalType: 'uint64',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'addEuint8',
    inputs: [
      {
        name: 'inputEuint8',
        type: 'bytes32',
        internalType: 'externalEuint8',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint8',
        internalType: 'uint8',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'confidentialProtocolId',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'uint256',
        internalType: 'uint256',
      },
    ],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    name: 'createPublicHandle',
    inputs: [
      {
        name: 'inputHandle',
        type: 'bytes32',
        internalType: 'bytes32',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'eqEaddress',
    inputs: [
      {
        name: 'inputEaddress',
        type: 'bytes32',
        internalType: 'externalEaddress',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'address',
        internalType: 'address',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'getClearText',
    inputs: [
      {
        name: 'handle',
        type: 'bytes32',
        internalType: 'bytes32',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'uint256',
        internalType: 'uint256',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getCoprocessorConfig',
    inputs: [],
    outputs: [
      {
        name: 'config',
        type: 'tuple',
        internalType: 'struct CoprocessorConfig',
        components: [
          {
            name: 'ACLAddress',
            type: 'address',
            internalType: 'address',
          },
          {
            name: 'CoprocessorAddress',
            type: 'address',
            internalType: 'address',
          },
          {
            name: 'KMSVerifierAddress',
            type: 'address',
            internalType: 'address',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEaddress',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'eaddress',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEaddressOf',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'eaddress',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEbool',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'ebool',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEboolOf',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'ebool',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint128',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint128',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint128Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint128',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint16',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint16',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint16Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint16',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint256',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint256',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint256Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint256',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint32',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint32',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint32Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint32',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint64',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint64',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint64Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint64',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint8',
    inputs: [],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint8',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getEuint8Of',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint8',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getHandle',
    inputs: [
      {
        name: 'fheType',
        type: 'uint8',
        internalType: 'enum FheType',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'bytes32',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'getHandleOf',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
      {
        name: 'fheType',
        type: 'uint8',
        internalType: 'enum FheType',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'bytes32',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'hasClearText',
    inputs: [
      {
        name: 'handle',
        type: 'bytes32',
        internalType: 'bytes32',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'hasHandleOf',
    inputs: [
      {
        name: 'account',
        type: 'address',
        internalType: 'address',
      },
      {
        name: 'fheType',
        type: 'uint8',
        internalType: 'enum FheType',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'makePubliclyDecryptable',
    inputs: [
      {
        name: 'fheType',
        type: 'uint8',
        internalType: 'enum FheType',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEaddress',
    inputs: [
      {
        name: 'addr',
        type: 'address',
        internalType: 'address',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'eaddress',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEbool',
    inputs: [
      {
        name: 'value',
        type: 'bool',
        internalType: 'bool',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'ebool',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint128',
    inputs: [
      {
        name: 'value',
        type: 'uint128',
        internalType: 'uint128',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint128',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint16',
    inputs: [
      {
        name: 'value',
        type: 'uint16',
        internalType: 'uint16',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint16',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint256',
    inputs: [
      {
        name: 'value',
        type: 'uint256',
        internalType: 'uint256',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint256',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint32',
    inputs: [
      {
        name: 'value',
        type: 'uint32',
        internalType: 'uint32',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint32',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint64',
    inputs: [
      {
        name: 'value',
        type: 'uint64',
        internalType: 'uint64',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint64',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setClearEuint8',
    inputs: [
      {
        name: 'value',
        type: 'uint8',
        internalType: 'uint8',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setCoprocessorConfig',
    inputs: [
      {
        name: 'config',
        type: 'tuple',
        internalType: 'struct CoprocessorConfig',
        components: [
          {
            name: 'ACLAddress',
            type: 'address',
            internalType: 'address',
          },
          {
            name: 'CoprocessorAddress',
            type: 'address',
            internalType: 'address',
          },
          {
            name: 'KMSVerifierAddress',
            type: 'address',
            internalType: 'address',
          },
        ],
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEaddress',
    inputs: [
      {
        name: 'inputEaddress',
        type: 'bytes32',
        internalType: 'externalEaddress',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'address',
        internalType: 'address',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'eaddress',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEbool',
    inputs: [
      {
        name: 'inputEbool',
        type: 'bytes32',
        internalType: 'externalEbool',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'bool',
        internalType: 'bool',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'ebool',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint128',
    inputs: [
      {
        name: 'inputEuint128',
        type: 'bytes32',
        internalType: 'externalEuint128',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint128',
        internalType: 'uint128',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint128',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint16',
    inputs: [
      {
        name: 'inputEuint16',
        type: 'bytes32',
        internalType: 'externalEuint16',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint16',
        internalType: 'uint16',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint16',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint256',
    inputs: [
      {
        name: 'inputEuint256',
        type: 'bytes32',
        internalType: 'externalEuint256',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint256',
        internalType: 'uint256',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint256',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint32',
    inputs: [
      {
        name: 'inputEuint32',
        type: 'bytes32',
        internalType: 'externalEuint32',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint32',
        internalType: 'uint32',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint32',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint64',
    inputs: [
      {
        name: 'inputEuint64',
        type: 'bytes32',
        internalType: 'externalEuint64',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint64',
        internalType: 'uint64',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint64',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'setEuint8',
    inputs: [
      {
        name: 'inputEuint8',
        type: 'bytes32',
        internalType: 'externalEuint8',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint8',
        internalType: 'uint8',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [
      {
        name: '',
        type: 'bytes32',
        internalType: 'euint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'verify',
    inputs: [
      {
        name: 'handlesList',
        type: 'bytes32[]',
        internalType: 'bytes32[]',
      },
      {
        name: 'cleartexts',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'decryptionProof',
        type: 'bytes',
        internalType: 'bytes',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'xorEbool',
    inputs: [
      {
        name: 'inputEbool',
        type: 'bytes32',
        internalType: 'externalEbool',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'bool',
        internalType: 'bool',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'xorEuint256',
    inputs: [
      {
        name: 'inputEuint256',
        type: 'bytes32',
        internalType: 'externalEuint256',
      },
      {
        name: 'inputProof',
        type: 'bytes',
        internalType: 'bytes',
      },
      {
        name: 'clearValue',
        type: 'uint256',
        internalType: 'uint256',
      },
      {
        name: 'makePublic',
        type: 'bool',
        internalType: 'bool',
      },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    name: 'PublicDecryptionVerified',
    inputs: [
      {
        name: 'handlesList',
        type: 'bytes32[]',
        indexed: false,
        internalType: 'bytes32[]',
      },
      {
        name: 'abiEncodedCleartexts',
        type: 'bytes',
        indexed: false,
        internalType: 'bytes',
      },
    ],
    anonymous: false,
  },
  {
    type: 'error',
    name: 'InvalidKMSSignatures',
    inputs: [],
  },
  {
    type: 'error',
    name: 'SenderNotAllowedToUseHandle',
    inputs: [
      {
        name: 'handle',
        type: 'bytes32',
        internalType: 'bytes32',
      },
      {
        name: 'sender',
        type: 'address',
        internalType: 'address',
      },
    ],
  },
] as const;
