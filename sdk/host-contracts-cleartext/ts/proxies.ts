////////////////////////////////////////////////////////////////////////////////

import { abi as erc1967ProxyAbi, template as erc1967ProxyTemplate } from './artifacts/ERC1967Proxy.js';
import { abi as emptyUUPSProxyAbi, template as emptyUUPSProxyTemplate } from './artifacts/EmptyUUPSProxy.js';
import { abi as emptyUUPSProxyACLAbi, template as emptyUUPSProxyACLTemplate } from './artifacts/EmptyUUPSProxyACL.js';
import type { ContractArtifact } from './private.js';
import type { AbstractEthereumSigner, AbstractEthereumUtils, DeployReturnType } from './public.js';
import { patchTemplateBytecode } from './utils.js';

////////////////////////////////////////////////////////////////////////////////

export function getEmptyUUPSProxyACLArtifact(): ContractArtifact {
  return {
    abi: emptyUUPSProxyACLAbi,
    bytecode: emptyUUPSProxyACLTemplate.bytecode,
    deployedBytecode: emptyUUPSProxyACLTemplate.deployedBytecode,
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the shared `EmptyUUPSProxy` implementation artifact with its hardcoded ACL address patched.
 *
 * `src/contracts/emptyProxy/EmptyUUPSProxy.sol` inherits from `ACLOwnable`, which reads the ACL address from
 * `FHEVMHostAddresses.sol` at compile time. Because this package deploys from bytecode templates, the compiled dummy
 * `ACL_ADDRESS` placeholder must be replaced with the caller's precomputed ACL proxy address before deployment.
 */
function getEmptyUUPSProxyArtifact(parameters: { readonly aclAddress: string }): ContractArtifact {
  const replacements = [{ referenceName: 'ACL_ADDRESS', replacement: parameters.aclAddress }];

  return {
    abi: emptyUUPSProxyAbi,
    bytecode: patchTemplateBytecode({
      template: emptyUUPSProxyTemplate,
      field: 'bytecode',
      replacements,
    }),
    deployedBytecode: patchTemplateBytecode({
      template: emptyUUPSProxyTemplate,
      field: 'deployedBytecode',
      replacements,
    }),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function getERC1967ProxyArtifact(): ContractArtifact {
  return {
    abi: erc1967ProxyAbi,
    bytecode: erc1967ProxyTemplate.bytecode,
    deployedBytecode: erc1967ProxyTemplate.deployedBytecode,
  };
}

////////////////////////////////////////////////////////////////////////////////

/*
  EmptyUUPSProxyACL.sol
*/
export async function deployEmptyUUPSProxyACL(parameters: {
  readonly deployer: AbstractEthereumSigner;
}): Promise<DeployReturnType> {
  const bytecode = getEmptyUUPSProxyACLArtifact().bytecode;
  return await parameters.deployer.deploy({ bytecode });
}

////////////////////////////////////////////////////////////////////////////////

/*
  EmptyUUPSProxy.sol
*/
export async function deployEmptyUUPSProxy(parameters: {
  readonly deployer: AbstractEthereumSigner;
  readonly aclAddress: string;
}): Promise<DeployReturnType> {
  const bytecode = getEmptyUUPSProxyArtifact(parameters).bytecode;
  return await parameters.deployer.deploy({ bytecode });
}

////////////////////////////////////////////////////////////////////////////////

/*
  ERC1967Proxy.sol + EmptyUUPSProxy.sol
*/
export async function deployERC1967Proxy(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly emptyUUPSProxyAddress: string;
}): Promise<DeployReturnType> {
  /*
    ERC1967Proxy proxy = new ERC1967Proxy(address(emptyUupsProxy), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
  */
  const erc1967ProxyArtifact = getERC1967ProxyArtifact();
  const initializeCall = await parameters.ethUtils.encodeCall({
    abi: emptyUUPSProxyAbi,
    functionName: 'initialize',
    args: [],
  });

  return await parameters.deployer.deploy({
    abi: erc1967ProxyArtifact.abi,
    bytecode: erc1967ProxyArtifact.bytecode,
    args: [parameters.emptyUUPSProxyAddress, initializeCall],
  });
}

////////////////////////////////////////////////////////////////////////////////

/*
  ERC1967Proxy.sol + EmptyUUPSProxyACL.sol
*/
export async function deployACLProxy(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly emptyUUPSProxyACLAddress: string;
}): Promise<DeployReturnType> {
  /*
    ERC1967Proxy proxy = new ERC1967Proxy(
        address(emptyUupsProxyACL), abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer))
    );
  */
  const erc1967ProxyArtifact = getERC1967ProxyArtifact();
  const initialOwner = await parameters.deployer.getAddress();
  const initializeCall = await parameters.ethUtils.encodeCall({
    abi: emptyUUPSProxyACLAbi,
    functionName: 'initialize',
    args: [initialOwner],
  });

  return await parameters.deployer.deploy({
    abi: erc1967ProxyArtifact.abi,
    bytecode: erc1967ProxyArtifact.bytecode,
    args: [parameters.emptyUUPSProxyACLAddress, initializeCall],
  });
}
