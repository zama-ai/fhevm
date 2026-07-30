export const confidentialFungibleTokenAbi = [
  {
    type: "function",
    name: "confidentialTransfer",
    inputs: [
      { name: "to", type: "address", internalType: "address" },
      {
        name: "encryptedAmount",
        type: "bytes32",
        internalType: "externalEuint64",
      },
      { name: "inputProof", type: "bytes", internalType: "bytes" },
    ],
    outputs: [{ name: "", type: "bytes32", internalType: "euint64" }],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "confidentialTransferFrom",
    inputs: [
      { name: "from", type: "address", internalType: "address" },
      { name: "to", type: "address", internalType: "address" },
      {
        name: "encryptedAmount",
        type: "bytes32",
        internalType: "externalEuint64",
      },
      { name: "inputProof", type: "bytes", internalType: "bytes" },
    ],
    outputs: [{ name: "", type: "bytes32", internalType: "euint64" }],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "confidentialBalanceOf",
    inputs: [{ name: "account", type: "address", internalType: "address" }],
    outputs: [{ name: "", type: "bytes32", internalType: "euint64" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "name",
    inputs: [],
    outputs: [{ name: "", type: "string", internalType: "string" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "symbol",
    inputs: [],
    outputs: [{ name: "", type: "string", internalType: "string" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "decimals",
    inputs: [],
    outputs: [{ name: "", type: "uint8", internalType: "uint8" }],
    stateMutability: "view",
  },
] as const;
