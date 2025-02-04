export const THFEExecutor = {
  _format: 'hh-sol-artifact-1',
  contractName: 'TFHEExecutor',
  sourceName: 'contracts/TFHEExecutor.sol',
  abi: [
    {
      inputs: [],
      stateMutability: 'nonpayable',
      type: 'constructor',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'handle',
          type: 'uint256',
        },
        {
          internalType: 'address',
          name: 'account',
          type: 'address',
        },
      ],
      name: 'ACLNotAllowed',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'target',
          type: 'address',
        },
      ],
      name: 'AddressEmptyCode',
      type: 'error',
    },
    {
      inputs: [],
      name: 'DivisionByZero',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'implementation',
          type: 'address',
        },
      ],
      name: 'ERC1967InvalidImplementation',
      type: 'error',
    },
    {
      inputs: [],
      name: 'ERC1967NonPayable',
      type: 'error',
    },
    {
      inputs: [],
      name: 'FailedCall',
      type: 'error',
    },
    {
      inputs: [],
      name: 'IncompatibleTypes',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'uint8',
          name: 'typeOf',
          type: 'uint8',
        },
        {
          internalType: 'uint256',
          name: 'length',
          type: 'uint256',
        },
      ],
      name: 'InvalidByteLength',
      type: 'error',
    },
    {
      inputs: [],
      name: 'InvalidInitialization',
      type: 'error',
    },
    {
      inputs: [],
      name: 'InvalidType',
      type: 'error',
    },
    {
      inputs: [],
      name: 'IsNotScalar',
      type: 'error',
    },
    {
      inputs: [],
      name: 'IsScalar',
      type: 'error',
    },
    {
      inputs: [],
      name: 'NotInitializing',
      type: 'error',
    },
    {
      inputs: [],
      name: 'NotPowerOfTwo',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'owner',
          type: 'address',
        },
      ],
      name: 'OwnableInvalidOwner',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'account',
          type: 'address',
        },
      ],
      name: 'OwnableUnauthorizedAccount',
      type: 'error',
    },
    {
      inputs: [],
      name: 'SecondOperandIsNotScalar',
      type: 'error',
    },
    {
      inputs: [],
      name: 'UUPSUnauthorizedCallContext',
      type: 'error',
    },
    {
      inputs: [
        {
          internalType: 'bytes32',
          name: 'slot',
          type: 'bytes32',
        },
      ],
      name: 'UUPSUnsupportedProxiableUUID',
      type: 'error',
    },
    {
      inputs: [],
      name: 'UnsupportedType',
      type: 'error',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'Cast',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheAdd',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheBitAnd',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheBitOr',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheBitXor',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheDiv',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheEq',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes',
          name: 'rhs',
          type: 'bytes',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheEqBytes',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheGe',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheGt',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'control',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'ifTrue',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'ifFalse',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheIfThenElse',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheLe',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheLt',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheMax',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheMin',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheMul',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheNe',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes',
          name: 'rhs',
          type: 'bytes',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheNeBytes',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheNeg',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheNot',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'randType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'bytes16',
          name: 'seed',
          type: 'bytes16',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheRand',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'upperBound',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'randType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'bytes16',
          name: 'seed',
          type: 'bytes16',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheRandBounded',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheRem',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheRotl',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheRotr',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheShl',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheShr',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'FheSub',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: false,
          internalType: 'uint64',
          name: 'version',
          type: 'uint64',
        },
      ],
      name: 'Initialized',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'previousOwner',
          type: 'address',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'newOwner',
          type: 'address',
        },
      ],
      name: 'OwnershipTransferStarted',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'previousOwner',
          type: 'address',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'newOwner',
          type: 'address',
        },
      ],
      name: 'OwnershipTransferred',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'pt',
          type: 'uint256',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'TrivialEncrypt',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'bytes',
          name: 'pt',
          type: 'bytes',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'TrivialEncryptBytes',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'implementation',
          type: 'address',
        },
      ],
      name: 'Upgraded',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'address',
          name: 'caller',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'bytes32',
          name: 'inputHandle',
          type: 'bytes32',
        },
        {
          indexed: false,
          internalType: 'address',
          name: 'userAddress',
          type: 'address',
        },
        {
          indexed: false,
          internalType: 'bytes',
          name: 'inputProof',
          type: 'bytes',
        },
        {
          indexed: false,
          internalType: 'bytes1',
          name: 'inputType',
          type: 'bytes1',
        },
        {
          indexed: false,
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      name: 'VerifyCiphertext',
      type: 'event',
    },
    {
      inputs: [],
      name: 'HANDLE_VERSION',
      outputs: [
        {
          internalType: 'uint8',
          name: '',
          type: 'uint8',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'UPGRADE_INTERFACE_VERSION',
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
      inputs: [],
      name: 'acceptOwnership',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
      ],
      name: 'cast',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheAdd',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheBitAnd',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheBitOr',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheBitXor',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheDiv',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheEq',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes',
          name: 'rhs',
          type: 'bytes',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheEq',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheGe',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheGt',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'control',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'ifTrue',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'ifFalse',
          type: 'uint256',
        },
      ],
      name: 'fheIfThenElse',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheLe',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheLt',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheMax',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheMin',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheMul',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes',
          name: 'rhs',
          type: 'bytes',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheNe',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheNe',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
      ],
      name: 'fheNeg',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'ct',
          type: 'uint256',
        },
      ],
      name: 'fheNot',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'bytes1',
          name: 'randType',
          type: 'bytes1',
        },
      ],
      name: 'fheRand',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'upperBound',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'randType',
          type: 'bytes1',
        },
      ],
      name: 'fheRandBounded',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheRem',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheRotl',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheRotr',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheShl',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheShr',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'lhs',
          type: 'uint256',
        },
        {
          internalType: 'uint256',
          name: 'rhs',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'scalarByte',
          type: 'bytes1',
        },
      ],
      name: 'fheSub',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [],
      name: 'getACLAddress',
      outputs: [
        {
          internalType: 'address',
          name: '',
          type: 'address',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'getFHEGasLimitAddress',
      outputs: [
        {
          internalType: 'address',
          name: '',
          type: 'address',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'getInputVerifierAddress',
      outputs: [
        {
          internalType: 'address',
          name: '',
          type: 'address',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'getVersion',
      outputs: [
        {
          internalType: 'string',
          name: '',
          type: 'string',
        },
      ],
      stateMutability: 'pure',
      type: 'function',
    },
    {
      inputs: [],
      name: 'owner',
      outputs: [
        {
          internalType: 'address',
          name: '',
          type: 'address',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'pendingOwner',
      outputs: [
        {
          internalType: 'address',
          name: '',
          type: 'address',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'proxiableUUID',
      outputs: [
        {
          internalType: 'bytes32',
          name: '',
          type: 'bytes32',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'renounceOwnership',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'newOwner',
          type: 'address',
        },
      ],
      name: 'transferOwnership',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'bytes',
          name: 'pt',
          type: 'bytes',
        },
        {
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
      ],
      name: 'trivialEncrypt',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'uint256',
          name: 'pt',
          type: 'uint256',
        },
        {
          internalType: 'bytes1',
          name: 'toType',
          type: 'bytes1',
        },
      ],
      name: 'trivialEncrypt',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'address',
          name: 'newImplementation',
          type: 'address',
        },
        {
          internalType: 'bytes',
          name: 'data',
          type: 'bytes',
        },
      ],
      name: 'upgradeToAndCall',
      outputs: [],
      stateMutability: 'payable',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'bytes32',
          name: 'inputHandle',
          type: 'bytes32',
        },
        {
          internalType: 'address',
          name: 'userAddress',
          type: 'address',
        },
        {
          internalType: 'bytes',
          name: 'inputProof',
          type: 'bytes',
        },
        {
          internalType: 'bytes1',
          name: 'inputType',
          type: 'bytes1',
        },
      ],
      name: 'verifyCiphertext',
      outputs: [
        {
          internalType: 'uint256',
          name: 'result',
          type: 'uint256',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'function',
    },
  ],
  linkReferences: {},
  deployedLinkReferences: {},
} as const
