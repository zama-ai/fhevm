export const relayerSdkTestAbi = [
  {
    inputs: [
      { internalType: "externalEaddress", name: "encryptedAddress", type: "bytes32" },
      { internalType: "bytes", name: "inputProof", type: "bytes" },
    ],
    name: "makePubliclyDecryptableExternalEaddress",
    outputs: [{ internalType: "eaddress", name: "xAddress", type: "bytes32" }],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      { internalType: "externalEbool", name: "encryptedBool", type: "bytes32" },
      { internalType: "bytes", name: "inputProof", type: "bytes" },
    ],
    name: "makePubliclyDecryptableExternalEbool",
    outputs: [{ internalType: "ebool", name: "xBool", type: "bytes32" }],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      { internalType: "externalEuint128", name: "encryptedUint128", type: "bytes32" },
      { internalType: "bytes", name: "inputProof", type: "bytes" },
    ],
    name: "makePubliclyDecryptableExternalEuint128",
    outputs: [{ internalType: "euint128", name: "xUint128", type: "bytes32" }],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      { internalType: "externalEuint8", name: "encryptedUint8", type: "bytes32" },
      { internalType: "bytes", name: "inputProof", type: "bytes" },
    ],
    name: "makePubliclyDecryptableExternalEuint8",
    outputs: [{ internalType: "euint8", name: "xUint8", type: "bytes32" }],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      { internalType: "externalEbool", name: "encryptedBool", type: "bytes32" },
      { internalType: "externalEuint8", name: "encryptedUint8", type: "bytes32" },
      { internalType: "externalEuint128", name: "encryptedUint128", type: "bytes32" },
      { internalType: "externalEaddress", name: "encryptedAddress", type: "bytes32" },
      { internalType: "bytes", name: "inputProof", type: "bytes" },
    ],
    name: "makePubliclyDecryptableExternalMixed",
    outputs: [
      { internalType: "ebool", name: "xBool", type: "bytes32" },
      { internalType: "euint8", name: "xUint8", type: "bytes32" },
      { internalType: "euint128", name: "xUint128", type: "bytes32" },
      { internalType: "eaddress", name: "xAddress", type: "bytes32" },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const;
