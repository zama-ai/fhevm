////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor ABI
////////////////////////////////////////////////////////////////////////////////

export const FhevmExecutorPartialAbi: Array<Record<string, unknown>> = [
  {
    inputs: [],
    stateMutability: "nonpayable",
    type: "constructor",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "handle",
        type: "bytes32",
      },
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
    ],
    name: "ACLNotAllowed",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "target",
        type: "address",
      },
    ],
    name: "AddressEmptyCode",
    type: "error",
  },
  {
    inputs: [],
    name: "DivisionByZero",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "implementation",
        type: "address",
      },
    ],
    name: "ERC1967InvalidImplementation",
    type: "error",
  },
  {
    inputs: [],
    name: "ERC1967NonPayable",
    type: "error",
  },
  {
    inputs: [],
    name: "FailedCall",
    type: "error",
  },
  {
    inputs: [],
    name: "IncompatibleTypes",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "enum FheType",
        name: "typeOf",
        type: "uint8",
      },
      {
        internalType: "uint256",
        name: "length",
        type: "uint256",
      },
    ],
    name: "InvalidByteLength",
    type: "error",
  },
  {
    inputs: [],
    name: "InvalidInitialization",
    type: "error",
  },
  {
    inputs: [],
    name: "InvalidType",
    type: "error",
  },
  {
    inputs: [],
    name: "IsNotScalar",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "sender",
        type: "address",
      },
    ],
    name: "NotHostOwner",
    type: "error",
  },
  {
    inputs: [],
    name: "NotInitializing",
    type: "error",
  },
  {
    inputs: [],
    name: "NotInitializingFromEmptyProxy",
    type: "error",
  },
  {
    inputs: [],
    name: "NotPowerOfTwo",
    type: "error",
  },
  {
    inputs: [],
    name: "ScalarByteIsNotBoolean",
    type: "error",
  },
  {
    inputs: [],
    name: "SecondOperandIsNotScalar",
    type: "error",
  },
  {
    inputs: [],
    name: "UUPSUnauthorizedCallContext",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "slot",
        type: "bytes32",
      },
    ],
    name: "UUPSUnsupportedProxiableUUID",
    type: "error",
  },
  {
    inputs: [],
    name: "UnsupportedType",
    type: "error",
  },
  {
    inputs: [],
    name: "UpperBoundAboveMaxTypeValue",
    type: "error",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "enum FheType",
        name: "toType",
        type: "uint8",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "Cast",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheAdd",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheBitAnd",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheBitOr",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheBitXor",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheDiv",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheEq",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheGe",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheGt",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "control",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "ifTrue",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "ifFalse",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheIfThenElse",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheLe",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheLt",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheMax",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheMin",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheMul",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheNe",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheNeg",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheNot",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "enum FheType",
        name: "randType",
        type: "uint8",
      },
      {
        indexed: false,
        internalType: "bytes16",
        name: "seed",
        type: "bytes16",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheRand",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "upperBound",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "enum FheType",
        name: "randType",
        type: "uint8",
      },
      {
        indexed: false,
        internalType: "bytes16",
        name: "seed",
        type: "bytes16",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheRandBounded",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheRem",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheRotl",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheRotr",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheShl",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheShr",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "FheSub",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint64",
        name: "version",
        type: "uint64",
      },
    ],
    name: "Initialized",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "pt",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "enum FheType",
        name: "toType",
        type: "uint8",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "TrivialEncrypt",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "implementation",
        type: "address",
      },
    ],
    name: "Upgraded",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "caller",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "inputHandle",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "address",
        name: "userAddress",
        type: "address",
      },
      {
        indexed: false,
        internalType: "bytes",
        name: "inputProof",
        type: "bytes",
      },
      {
        indexed: false,
        internalType: "enum FheType",
        name: "inputType",
        type: "uint8",
      },
      {
        indexed: false,
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    name: "VerifyInput",
    type: "event",
  },
  {
    inputs: [],
    name: "UPGRADE_INTERFACE_VERSION",
    outputs: [
      {
        internalType: "string",
        name: "",
        type: "string",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
      {
        internalType: "enum FheType",
        name: "toType",
        type: "uint8",
      },
    ],
    name: "cast",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheAdd",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheBitAnd",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheBitOr",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheBitXor",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheDiv",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheEq",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheGe",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheGt",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "control",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "ifTrue",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "ifFalse",
        type: "bytes32",
      },
    ],
    name: "fheIfThenElse",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheLe",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheLt",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheMax",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheMin",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheMul",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheNe",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
    ],
    name: "fheNeg",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "ct",
        type: "bytes32",
      },
    ],
    name: "fheNot",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "enum FheType",
        name: "randType",
        type: "uint8",
      },
    ],
    name: "fheRand",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "upperBound",
        type: "uint256",
      },
      {
        internalType: "enum FheType",
        name: "randType",
        type: "uint8",
      },
    ],
    name: "fheRandBounded",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheRem",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheRotl",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheRotr",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheShl",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheShr",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "lhs",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "rhs",
        type: "bytes32",
      },
      {
        internalType: "bytes1",
        name: "scalarByte",
        type: "bytes1",
      },
    ],
    name: "fheSub",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "getACLAddress",
    outputs: [
      {
        internalType: "address",
        name: "",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "getHCULimitAddress",
    outputs: [
      {
        internalType: "address",
        name: "",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "getHandleVersion",
    outputs: [
      {
        internalType: "uint8",
        name: "",
        type: "uint8",
      },
    ],
    stateMutability: "pure",
    type: "function",
  },
  {
    inputs: [],
    name: "getInputVerifierAddress",
    outputs: [
      {
        internalType: "address",
        name: "",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "getVersion",
    outputs: [
      {
        internalType: "string",
        name: "",
        type: "string",
      },
    ],
    stateMutability: "pure",
    type: "function",
  },
  {
    inputs: [],
    name: "initializeFromEmptyProxy",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "proxiableUUID",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "pt",
        type: "uint256",
      },
      {
        internalType: "enum FheType",
        name: "toType",
        type: "uint8",
      },
    ],
    name: "trivialEncrypt",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "newImplementation",
        type: "address",
      },
      {
        internalType: "bytes",
        name: "data",
        type: "bytes",
      },
    ],
    name: "upgradeToAndCall",
    outputs: [],
    stateMutability: "payable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "inputHandle",
        type: "bytes32",
      },
      {
        internalType: "address",
        name: "userAddress",
        type: "address",
      },
      {
        internalType: "bytes",
        name: "inputProof",
        type: "bytes",
      },
      {
        internalType: "enum FheType",
        name: "inputType",
        type: "uint8",
      },
    ],
    name: "verifyInput",
    outputs: [
      {
        internalType: "bytes32",
        name: "result",
        type: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
];
