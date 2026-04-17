export const FHETestAddresses = {
  mainnet: '0x7553CB9124f974Ee475E5cE45482F90d5B6076BC',
  testnet: '0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241',
  devnet: '0x54ED156e4c905598eF34BBDa7b4a04A008EE8bC2',
};

export const FHETestABI = [
  {
    inputs: [],
    name: 'InvalidKMSSignatures',
    type: 'error',
  },
  {
    inputs: [
      {
        internalType: 'bytes32',
        name: 'handle',
        type: 'bytes32',
      },
      {
        internalType: 'address',
        name: 'sender',
        type: 'address',
      },
    ],
    name: 'SenderNotAllowedToUseHandle',
    type: 'error',
  },
  {
    inputs: [],
    name: 'ZamaProtocolUnsupported',
    type: 'error',
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: 'bytes32[]',
        name: 'handlesList',
        type: 'bytes32[]',
      },
      {
        indexed: false,
        internalType: 'bytes',
        name: 'abiEncodedCleartexts',
        type: 'bytes',
      },
    ],
    name: 'PublicDecryptionVerified',
    type: 'event',
  },
  {
    inputs: [],
    name: 'CONTRACT_NAME',
    outputs: [
      {
        internalType: 'string',
        name: '',
        type: 'string',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint128',
        name: 'inputEuint128',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'addEuint128',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint16',
        name: 'inputEuint16',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'addEuint16',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint32',
        name: 'inputEuint32',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'addEuint32',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint64',
        name: 'inputEuint64',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'addEuint64',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint8',
        name: 'inputEuint8',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'addEuint8',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'confidentialProtocolId',
    outputs: [
      {
        internalType: 'uint256',
        name: '',
        type: 'uint256',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getCoprocessorConfig',
    outputs: [
      {
        components: [
          {
            internalType: 'address',
            name: 'ACLAddress',
            type: 'address',
          },
          {
            internalType: 'address',
            name: 'CoprocessorAddress',
            type: 'address',
          },
          {
            internalType: 'address',
            name: 'KMSVerifierAddress',
            type: 'address',
          },
        ],
        internalType: 'struct CoprocessorConfig',
        name: 'config',
        type: 'tuple',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEaddress',
    outputs: [
      {
        internalType: 'eaddress',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEbool',
    outputs: [
      {
        internalType: 'ebool',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint128',
    outputs: [
      {
        internalType: 'euint128',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint16',
    outputs: [
      {
        internalType: 'euint16',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint256',
    outputs: [
      {
        internalType: 'euint256',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint32',
    outputs: [
      {
        internalType: 'euint32',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint64',
    outputs: [
      {
        internalType: 'euint64',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'getEuint8',
    outputs: [
      {
        internalType: 'euint8',
        name: '',
        type: 'bytes32',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEaddress',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEbool',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint128',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint16',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint256',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint32',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint64',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'makePubliclyDecryptableEuint8',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEbool',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint128',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint16',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint256',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint32',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint64',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'randEuint8',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'bytes32[]',
        name: 'handlesList',
        type: 'bytes32[]',
      },
      {
        internalType: 'bytes',
        name: 'cleartexts',
        type: 'bytes',
      },
      {
        internalType: 'bytes',
        name: 'decryptionProof',
        type: 'bytes',
      },
    ],
    name: 'verify',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEbool',
        name: 'inputEbool',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'xorEbool',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'externalEuint256',
        name: 'inputEuint256',
        type: 'bytes32',
      },
      {
        internalType: 'bytes',
        name: 'inputProof',
        type: 'bytes',
      },
    ],
    name: 'xorEuint256',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
];
