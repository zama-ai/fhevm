import type { BootstrapConfigV12 } from './types/public.js';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
export const DEFAULT_INPUT_VERIFICATION_ADDRESS = '0x6189F6c0c3E40B4a3c72ec86262295D78d845297';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
export const DEFAULT_DECRYPTION_ADDRESS = '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721';

export const DEFAULT_HCU_CAP_PER_BLOCK = 281474976710655n;
export const DEFAULT_MAX_HCU_DEPTH_PER_TX = 5000000n;
export const DEFAULT_MAX_HCU_PER_TX = 20000000n;

export const DEFAULT_CHAIN_ID_GATEWAY = 654321n;

export const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';

export const DEFAULT_COPROCESSORS_MNEMONIC = FHEVM_MNEMONIC;
export const DEFAULT_COPROCESSORS_MNEMONIC_PATH = "m/44'/60'/0'/2/";
export const DEFAULT_COPROCESSORS_MNEMONIC_INDEX = 0;

export const DEFAULT_KMS_NODES_MNEMONIC = FHEVM_MNEMONIC;
export const DEFAULT_KMS_NODES_MNEMONIC_PATH = "m/44'/60'/0'/3/";
export const DEFAULT_KMS_NODES_MNEMONIC_INDEX = 0;

export const DEFAULT_KMS_THRESHOLD = 1n;
export const DEFAULT_COPROCESSOR_THRESHOLD = 1n;

export const DEFAULT_NUM_COPROCESSORS = 4n;
export const DEFAULT_NUM_KMS_NODES = 4n;

// cast wallet private-key \
//  --mnemonic "test test test test test test test future home engine virtual motion" \
//  --mnemonic-derivation-path "m/44'/60'/0'/2/0"
export const DEFAULT_COPROCESSOR_PK = [
  '0xcae2ef860129ace577171fcfd131fe77bf849460f05b0b128ecfb75b53541ecc',
  '0x3729cf7a85b557b3381599dd71fc446d2b33c39c69a2d3c385d8217fca97f345',
  '0x692c60f3f7e8fea712a3700106a21ad211f5739f1d01eed7ae751cff7bf3de9c',
  '0xbedfb2aafa800c095df023b4278688eb9f9f11779cf5465e4b5e8ddc3d82f278',
];

export const DEFAULT_COPROCESSOR_ADDRESS = [
  '0x6727b56B5D1E990C75fE5dCd572D084fc5DC35cb',
  '0xF1CDC3ebe04A063EAbfaf5c3129494F6049Ee229',
  '0x46A03377bcBE895407ee4842b63f3249bc8327BD',
  '0xFd0ea258B3D2b153DD76CAC0E2270574Da8f18E8',
];

// cast wallet private-key \
//  --mnemonic "test test test test test test test future home engine virtual motion" \
//  --mnemonic-derivation-path "m/44'/60'/0'/3/0"
export const DEFAULT_KMS_NODE_PK = [
  '0x57116cca6395f3703b807ae186ce23815f830496ccb10038a877b23ded9cbcd6',
  '0x8ed011f8caa4e207060a959d9afa0fd519bd6436db13f55c029aa6ede60c91d4',
  '0xb4ec522c2bfc8f6ba374c41d6077d4d090af73db404983b5e5222ca97a77456e',
  '0x76ec29ad59aa785aa38606d9ae40585620b06d95e7c8eee87b439c21f72a6aa7',
];

export const DEFAULT_KMS_NODE_ADDRESS = [
  '0x649c125E183640141C2AC329673A22dF9FA81b3C',
  '0xDB05A2EdF583E3b1644A59434a163aD178595EAf',
  '0x03D684498fB1D52286D09Dd42e720c89b55C63c3',
  '0x8252dD4D1350BCd34f419aBD05f8cfA146d71C5D',
];

export const DEFAULT_BOOTSTRAP_CONFIG_V12: BootstrapConfigV12 = {
  hcuLimit: {
    hcuCapPerBlock: DEFAULT_HCU_CAP_PER_BLOCK,
    maxHCUDepthPerTx: DEFAULT_MAX_HCU_DEPTH_PER_TX,
    maxHCUPerTx: DEFAULT_MAX_HCU_PER_TX,
  },
  inputVerifier: {
    chainIDSource: DEFAULT_CHAIN_ID_GATEWAY,
    initialSigners: DEFAULT_COPROCESSOR_ADDRESS,
    initialThreshold: DEFAULT_COPROCESSOR_THRESHOLD,
    verifyingContractSource: DEFAULT_INPUT_VERIFICATION_ADDRESS,
  },
  kmsVerifier: {
    chainIDSource: DEFAULT_CHAIN_ID_GATEWAY,
    initialSigners: DEFAULT_KMS_NODE_ADDRESS,
    initialThreshold: DEFAULT_KMS_THRESHOLD,
    verifyingContractSource: DEFAULT_DECRYPTION_ADDRESS,
  },
};
