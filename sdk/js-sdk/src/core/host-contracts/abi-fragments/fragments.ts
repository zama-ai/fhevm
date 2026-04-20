////////////////////////////////////////////////////////////////////////////////
// <AnyHostContract>.getVersion()
////////////////////////////////////////////////////////////////////////////////

export const getVersionAbi: readonly [Record<string, unknown> & { readonly name: 'getVersion' }] = [
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.getFHEVMExecutorAddress()
////////////////////////////////////////////////////////////////////////////////

export const getFHEVMExecutorAddressAbi: readonly [
  Record<string, unknown> & { readonly name: 'getFHEVMExecutorAddress' },
] = [
  {
    inputs: [],
    name: 'getFHEVMExecutorAddress',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.delegateForUserDecryption()
////////////////////////////////////////////////////////////////////////////////

export const delegateForUserDecryptionAbi: readonly [
  Record<string, unknown> & { readonly name: 'delegateForUserDecryption' },
] = [
  {
    inputs: [
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
      { internalType: 'uint64', name: 'expirationDate', type: 'uint64' },
    ],
    name: 'delegateForUserDecryption',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.revokeDelegationForUserDecryption()
////////////////////////////////////////////////////////////////////////////////

export const revokeDelegationForUserDecryptionAbi: readonly [
  Record<string, unknown> & { readonly name: 'revokeDelegationForUserDecryption' },
] = [
  {
    inputs: [
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
    ],
    name: 'revokeDelegationForUserDecryption',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.getUserDecryptionDelegationExpirationDate()
////////////////////////////////////////////////////////////////////////////////

export const getUserDecryptionDelegationExpirationDateAbi: readonly [
  Record<string, unknown> & {
    readonly name: 'getUserDecryptionDelegationExpirationDate';
  },
] = [
  {
    inputs: [
      { internalType: 'address', name: 'delegator', type: 'address' },
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
    ],
    name: 'getUserDecryptionDelegationExpirationDate',
    outputs: [{ internalType: 'uint64', name: '', type: 'uint64' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.isHandleDelegatedForUserDecryption()
////////////////////////////////////////////////////////////////////////////////

export const isHandleDelegatedForUserDecryptionAbi: readonly [
  Record<string, unknown> & {
    readonly name: 'isHandleDelegatedForUserDecryption';
  },
] = [
  {
    inputs: [
      { internalType: 'address', name: 'delegator', type: 'address' },
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
      { internalType: 'bytes32', name: 'handle', type: 'bytes32' },
    ],
    name: 'isHandleDelegatedForUserDecryption',
    outputs: [{ internalType: 'bool', name: '', type: 'bool' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor.getACLAddress()
////////////////////////////////////////////////////////////////////////////////

export const getACLAddressAbi: readonly [Record<string, unknown> & { readonly name: 'getACLAddress' }] = [
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor.getHCULimitAddress()
////////////////////////////////////////////////////////////////////////////////

export const getHCULimitAddressAbi: readonly [Record<string, unknown> & { readonly name: 'getHCULimitAddress' }] = [
  {
    inputs: [],
    name: 'getHCULimitAddress',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor.getHandleVersion()
////////////////////////////////////////////////////////////////////////////////

export const getHandleVersionAbi: readonly [Record<string, unknown> & { readonly name: 'getHandleVersion' }] = [
  {
    inputs: [],
    name: 'getHandleVersion',
    outputs: [
      {
        internalType: 'uint8',
        name: '',
        type: 'uint8',
      },
    ],
    stateMutability: 'pure',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor.getInputVerifierAddress()
////////////////////////////////////////////////////////////////////////////////

export const getInputVerifierAddressAbi: readonly [
  Record<string, unknown> & { readonly name: 'getInputVerifierAddress' },
] = [
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.eip712Domain()
// InputVerifier.eip712Domain()
////////////////////////////////////////////////////////////////////////////////

export const eip712DomainAbi: readonly [Record<string, unknown> & { readonly name: 'eip712Domain' }] = [
  {
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      {
        internalType: 'bytes1',
        name: 'fields',
        type: 'bytes1',
      },
      {
        internalType: 'string',
        name: 'name',
        type: 'string',
      },
      {
        internalType: 'string',
        name: 'version',
        type: 'string',
      },
      {
        internalType: 'uint256',
        name: 'chainId',
        type: 'uint256',
      },
      {
        internalType: 'address',
        name: 'verifyingContract',
        type: 'address',
      },
      {
        internalType: 'bytes32',
        name: 'salt',
        type: 'bytes32',
      },
      {
        internalType: 'uint256[]',
        name: 'extensions',
        type: 'uint256[]',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.getThreshold()
// InputVerifier.getThreshold()
////////////////////////////////////////////////////////////////////////////////

export const getThresholdAbi: readonly [Record<string, unknown> & { readonly name: 'getThreshold' }] = [
  {
    inputs: [],
    name: 'getThreshold',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.getKmsSigners()
////////////////////////////////////////////////////////////////////////////////

export const getKmsSignersAbi: readonly [Record<string, unknown> & { readonly name: 'getKmsSigners' }] = [
  {
    inputs: [],
    name: 'getKmsSigners',
    outputs: [
      {
        internalType: 'address[]',
        name: '',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.getCurrentKmsContextId()
////////////////////////////////////////////////////////////////////////////////

export const getCurrentKmsContextIdAbi: readonly [
  Record<string, unknown> & { readonly name: 'getCurrentKmsContextId' },
] = [
  {
    inputs: [],
    name: 'getCurrentKmsContextId',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.getSignersForKmsContext()
////////////////////////////////////////////////////////////////////////////////

export const getSignersForKmsContextAbi: readonly [
  Record<string, unknown> & { readonly name: 'getSignersForKmsContext' },
] = [
  {
    inputs: [
      {
        internalType: 'uint256',
        name: 'kmsContextId',
        type: 'uint256',
      },
    ],
    name: 'getSignersForKmsContext',
    outputs: [
      {
        internalType: 'address[]',
        name: '',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

//getSignersForKmsContext
////////////////////////////////////////////////////////////////////////////////
// InputVerifier.getCoprocessorSigners()
////////////////////////////////////////////////////////////////////////////////

export const getCoprocessorSignersAbi: readonly [Record<string, unknown> & { readonly name: 'getCoprocessorSigners' }] =
  [
    {
      inputs: [],
      name: 'getCoprocessorSigners',
      outputs: [
        {
          internalType: 'address[]',
          name: '',
          type: 'address[]',
        },
      ],
      stateMutability: 'view',
      type: 'function',
    },
  ] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.persistAllowed(handle, account)
////////////////////////////////////////////////////////////////////////////////

export const persistAllowedAbi: readonly [Record<string, unknown> & { readonly name: 'persistAllowed' }] = [
  {
    inputs: [
      {
        internalType: 'bytes32',
        name: 'handle',
        type: 'bytes32',
      },
      {
        internalType: 'address',
        name: 'account',
        type: 'address',
      },
    ],
    name: 'persistAllowed',
    outputs: [
      {
        internalType: 'bool',
        name: '',
        type: 'bool',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// ACL.isAllowedForDecryption(handle)
////////////////////////////////////////////////////////////////////////////////

export const isAllowedForDecryptionAbi: readonly [
  Record<string, unknown> & { readonly name: 'isAllowedForDecryption' },
] = [
  {
    inputs: [
      {
        internalType: 'bytes32',
        name: 'handle',
        type: 'bytes32',
      },
    ],
    name: 'isAllowedForDecryption',
    outputs: [
      {
        internalType: 'bool',
        name: '',
        type: 'bool',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// InputVerifier.EIP712_INPUT_VERIFICATION_TYPEHASH()
////////////////////////////////////////////////////////////////////////////////

export const eip712InputVerificationTypehashAbi: readonly [
  Record<string, unknown> & {
    readonly name: 'EIP712_INPUT_VERIFICATION_TYPEHASH';
  },
] = [
  {
    inputs: [],
    name: 'EIP712_INPUT_VERIFICATION_TYPEHASH',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.DECRYPTION_RESULT_TYPEHASH()
////////////////////////////////////////////////////////////////////////////////

export const decryptionResultTypehashAbi: readonly [
  Record<string, unknown> & {
    readonly name: 'DECRYPTION_RESULT_TYPEHASH';
  },
] = [
  {
    inputs: [],
    name: 'DECRYPTION_RESULT_TYPEHASH',
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
] as const;

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier.getContextSignersAndThresholdFromExtraData()
////////////////////////////////////////////////////////////////////////////////

export const getContextSignersAndThresholdFromExtraDataAbi: readonly [
  Record<string, unknown> & {
    readonly name: 'getContextSignersAndThresholdFromExtraData';
  },
] = [
  {
    type: 'function',
    name: 'getContextSignersAndThresholdFromExtraData',
    inputs: [{ name: 'extraData', type: 'bytes' }],
    outputs: [
      { name: 'signers', type: 'address[]' },
      { name: 'threshold', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
] as const;
