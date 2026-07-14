import fs from 'node:fs';
import path from 'node:path';

import { LEGACY_SDK_SELECTION_WARNING, selectSdk } from './selection';
import type { Auth, SdkInstance } from './types';

export type SdkConfig = {
  verifyingContractAddressDecryption: string;
  verifyingContractAddressInputVerification: string;
  kmsContractAddress: string;
  inputVerifierContractAddress: string;
  aclContractAddress: string;
  protocolConfigAddress?: string;
  relayerUrl: string;
  rpcUrl: string;
  gatewayChainId: number;
  chainId: number;
  auth?: Auth;
};

const selection = selectSdk(process.env);

type SdkNetworkDefaults = Omit<SdkConfig, 'rpcUrl' | 'auth' | 'protocolConfigAddress'>;

type FhevmChainDefaults = {
  id: number;
  fhevm: {
    contracts: {
      acl: { address: string };
      inputVerifier: { address: string };
      kmsVerifier: { address: string };
    };
    relayerUrl: string;
    gateway: {
      id: number;
      contracts: {
        decryption: { address: string };
        inputVerification: { address: string };
      };
    };
  };
};

function installedPackageVersion(packageName: string, entrypoint: string): string {
  let directory = path.dirname(require.resolve(entrypoint));
  while (directory !== path.dirname(directory)) {
    const manifestPath = path.join(directory, 'package.json');
    if (fs.existsSync(manifestPath)) {
      const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as { name?: string; version?: string };
      if (manifest.name === packageName && manifest.version) return manifest.version;
    }
    directory = path.dirname(directory);
  }
  throw new Error(`Could not resolve installed version for ${packageName}`);
}

const packageIdentity =
  selection.family === 'fhevm-sdk'
    ? { name: '@fhevm/sdk', version: installedPackageVersion('@fhevm/sdk', '@fhevm/sdk') }
    : {
        name: '@zama-fhe/relayer-sdk',
        version: installedPackageVersion('@zama-fhe/relayer-sdk', '@zama-fhe/relayer-sdk/node'),
      };

const packageEntrypoint = fs.realpathSync(
  require.resolve(selection.family === 'fhevm-sdk' ? '@fhevm/sdk' : '@zama-fhe/relayer-sdk/node'),
);
const vendoredFhevmSdk = fs.realpathSync(path.resolve(__dirname, '../../../../sdk/js-sdk'));
if (
  selection.family === 'fhevm-sdk' &&
  selection.source === 'npm' &&
  packageEntrypoint.startsWith(`${vendoredFhevmSdk}${path.sep}`)
) {
  throw new Error(
    `Requested ${packageIdentity.name}@${selection.requestedVersion} from npm, but it resolves to the vendored SDK workspace`,
  );
}

if (selection.source === 'npm' && packageIdentity.version !== selection.requestedVersion) {
  throw new Error(
    `Requested ${packageIdentity.name}@${selection.requestedVersion}, but the test image contains ${packageIdentity.name}@${packageIdentity.version}`,
  );
}

if (selection.legacy) {
  console.warn(LEGACY_SDK_SELECTION_WARNING);
}
console.log(
  `E2E SDK identity: ${JSON.stringify({
    family: selection.family,
    source: selection.source,
    requestedVersion: selection.requestedVersion,
    package: packageIdentity,
    legacySelection: selection.legacy,
  })}`,
);

export function sdkNetworkDefaults(network: 'mainnet' | 'sepolia'): SdkNetworkDefaults {
  if (selection.family === 'relayer-sdk') {
    const module = require('@zama-fhe/relayer-sdk/node') as {
      MainnetConfig: SdkNetworkDefaults;
      SepoliaConfig: SdkNetworkDefaults;
    };
    return network === 'mainnet' ? module.MainnetConfig : module.SepoliaConfig;
  }

  const module = require('@fhevm/sdk/chains') as { mainnet: FhevmChainDefaults; sepolia: FhevmChainDefaults };
  const chain = network === 'mainnet' ? module.mainnet : module.sepolia;
  return {
    aclContractAddress: chain.fhevm.contracts.acl.address,
    kmsContractAddress: chain.fhevm.contracts.kmsVerifier.address,
    inputVerifierContractAddress: chain.fhevm.contracts.inputVerifier.address,
    verifyingContractAddressDecryption: chain.fhevm.gateway.contracts.decryption.address,
    verifyingContractAddressInputVerification: chain.fhevm.gateway.contracts.inputVerification.address,
    relayerUrl: chain.fhevm.relayerUrl,
    gatewayChainId: chain.fhevm.gateway.id,
    chainId: chain.id,
  };
}

export async function createSdkInstance(config: SdkConfig): Promise<SdkInstance> {
  if (selection.family === 'fhevm-sdk') {
    const { FhevmSdk } = await import('./fhevm-sdk/sdk');
    return FhevmSdk.create(config);
  }
  const { RelayerSdk } = await import('./relayer-sdk/sdk');
  return RelayerSdk.create(config);
}
