# What we want

- no direct dependency on viem or ethers
- viem can be added as a devDependency
- all ethereum RPC calls should run through an abstract interface defined by module
- we want a set of ts modules to enable anybody to deploy a cleartext stack in pure TS
- as argument the caller will provide an abstract eth prodider that could work for std RPC calls or in-memory eth nodes like hardhat
- the deployment should support either: setCodeAt(address, bytecode) or deploy(bytecode)
- obviously setCodeAt is only available on dev nodes like anvil, hardhat node, hardhat in-memory node, etc.
- the typescript code should live in 'ts' folder

# The abstract interface

Below is a first draft of the minimum set of functions needed to run the deploy

```ts
export interface AbstractEthereumProvider {
  // no signer ?
  setCodeAt(address: string, bytecode: string): Promise<unknown>;
  // no signer
  encodeCall(args: unknown): Promise<unknown>;
}

export interface AbstractEthereumSigner {
  // attached to a signer
  deploy(bytecode: string): Promise<unknown>;
  // attached to a signer
  writeContract(args: unknown): Promise<unknown>;
}
```

# How to get the empty proxies bytecode

Use a template mechanism.

# How to create the templates + abi

The following contracts must deployable using a template technique

List of proxies:

- host-contracts-cleartext/src/contracts/ACL.sol
- host-contracts-cleartext/src/contracts/HCULimit.sol
- host-contracts-cleartext/src/cleartext/CleartextFHEVMExecutor.sol
- host-contracts-cleartext/src/cleartext/CleartextInputVerifier.sol
- host-contracts-cleartext/src/cleartext/CleartextKMSVerifier.sol

List on non-proxies:

- host-contracts-cleartext/src/contracts/immutable/PauserSet.sol

1. set dummy addresses in the config
2. clean and re-build solidity using forge
3. Parse individual contract bytecode and detect where in the bytecode the hardcoded addresses are located. This gives an offset.
4. With the byte offset we can generate any modified bytecode with any desired contract addresses.
5. keep the individual ABIs for future use.

Store result in:
./templates : jsons with templated bytecode
./abi : jsons with only abi

Test:

- apply original deployed addresses and test that the resulting bytecode + deployedBytecode are identical
- compile using different config addresses and test that, when using template mechanism we get the same bytecode

# The deploy mechanism

1. deploy empty proxies
2. on each proxy, call `upgradeToAndCall`. For example:

```Solidity
    function _setACLImplementation(address aclAddress, ACL aclImplementation) internal {
        UUPSUpgradeable(aclAddress)
            .upgradeToAndCall(address(aclImplementation), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
    }
```

# How to deploy bytecode in viem

Here is a simple example

```ts
const abi = [
  {
    type: 'constructor',
    inputs: [{ name: 'owner', type: 'address' }],
    stateMutability: 'nonpayable',
  },
] as const;

const bytecode = '0x6080604052...' as `0x${string}`;

const hash = await walletClient.deployContract({
  abi,
  bytecode,
  args: [account.address],
});

const receipt = await publicClient.waitForTransactionReceipt({ hash });

console.log('deployed at', receipt.contractAddress);
```

# Test plan

1. Add an npm script that builds the package tarball.
2. Add a consumer-style integration test under `./test/ts`.
3. The test must install and use only the generated tarball, not local source paths.
4. The test must spawn a fresh `anvil` process.
5. The test must implement `AbstractEthereumLib` using `viem`.
6. `viem` must remain a devDependency only.
7. The test must import the published TS helper API through `@fhevm/host-contracts-cleartext/ts`.
8. The test must run the deploy function from `ts/index.ts`.
   - For now this deploy function is still a dummy.
   - The test should be fully wired first, then the deploy implementation can be developed behind it.
9. After deployment, the test must call a few view functions to check that the cleartext Solidity stack is usable.
10. The test must always kill `anvil`, including on failure.
