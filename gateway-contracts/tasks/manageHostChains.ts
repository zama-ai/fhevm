import { task, types } from "hardhat/config";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

// Host chain lifecycle tasks for the GatewayConfig contract.
//
// `addHostChain` already has a dedicated task (see `addHostChains.ts`). These tasks close the
// symmetry by exposing the three remaining owner-only host chain operations: disable, enable and
// remove. They mirror the conventions of `addHostChains.ts`:
//   - `--use-internal-proxy-address` (default `false`) picks up `GATEWAY_CONFIG_ADDRESS` either from
//     env (operator / gitops flow) or from `addresses/.env.gateway` (local / CI flow).
//   - `DEPLOYER_PRIVATE_KEY` from env signs the transaction (the deployer owns GatewayConfig).

// Resolves the GatewayConfig contract connected to the deployer wallet, applying the same
// address-resolution rules as `addHostChains.ts`.
async function loadGatewayConfig(hre: any, useInternalProxyAddress: boolean) {
  await hre.run("compile:specific", { contract: "contracts" });

  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  if (useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  const proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");

  const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", proxyAddress, deployer);
  return { gatewayConfig, proxyAddress };
}

// Disable a registered host chain. The chain stays registered but is flagged as disabled.
task("task:disableHostChainOnGatewayConfig")
  .addParam("chainId", "The chain ID of the host chain to disable", undefined, types.string)
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ chainId, useInternalProxyAddress }, hre) {
    const { gatewayConfig, proxyAddress } = await loadGatewayConfig(hre, useInternalProxyAddress);

    // Pre-flight: surface a clean error before submitting a tx that would revert on-chain.
    if (!(await gatewayConfig.isHostChainRegistered(chainId))) {
      throw new Error(`Host chain ${chainId} is not registered in GatewayConfig at ${proxyAddress}.`);
    }
    if (await gatewayConfig.isHostChainDisabled(chainId)) {
      throw new Error(`Host chain ${chainId} is already disabled in GatewayConfig at ${proxyAddress}.`);
    }

    console.log(`Disabling host chain ${chainId} in GatewayConfig contract:`, proxyAddress, "\n");
    const tx = await gatewayConfig.disableHostChain(chainId);
    await tx.wait();

    console.log(`Emitted DisableHostChain(${chainId}) (tx: ${tx.hash})`);
    console.log("Host chain disabled!");
  });

// Re-enable a previously disabled host chain.
task("task:enableHostChainOnGatewayConfig")
  .addParam("chainId", "The chain ID of the host chain to enable", undefined, types.string)
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ chainId, useInternalProxyAddress }, hre) {
    const { gatewayConfig, proxyAddress } = await loadGatewayConfig(hre, useInternalProxyAddress);

    // Pre-flight: surface a clean error before submitting a tx that would revert on-chain.
    if (!(await gatewayConfig.isHostChainRegistered(chainId))) {
      throw new Error(`Host chain ${chainId} is not registered in GatewayConfig at ${proxyAddress}.`);
    }
    if (!(await gatewayConfig.isHostChainDisabled(chainId))) {
      throw new Error(`Host chain ${chainId} is already enabled in GatewayConfig at ${proxyAddress}.`);
    }

    console.log(`Enabling host chain ${chainId} in GatewayConfig contract:`, proxyAddress, "\n");
    const tx = await gatewayConfig.enableHostChain(chainId);
    await tx.wait();

    console.log(`Emitted EnableHostChain(${chainId}) (tx: ${tx.hash})`);
    console.log("Host chain enabled!");
  });

// Remove a host chain entirely. The chain must be disabled first, mirroring the on-chain
// `HostChainNotDisabled` guard, so the operator gets a clean error before submitting.
task("task:removeHostChainOnGatewayConfig")
  .addParam("chainId", "The chain ID of the host chain to remove", undefined, types.string)
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ chainId, useInternalProxyAddress }, hre) {
    const { gatewayConfig, proxyAddress } = await loadGatewayConfig(hre, useInternalProxyAddress);

    // Pre-flight: surface a clean error before submitting a tx that would revert on-chain.
    if (!(await gatewayConfig.isHostChainRegistered(chainId))) {
      throw new Error(`Host chain ${chainId} is not registered in GatewayConfig at ${proxyAddress}.`);
    }
    // Mirror the on-chain `HostChainNotDisabled` check (GatewayConfig.sol): a chain must be disabled
    // before it can be removed, so removal cannot pull a chain out from under in-flight requests.
    if (!(await gatewayConfig.isHostChainDisabled(chainId))) {
      throw new Error(
        `Host chain ${chainId} must be disabled before removal. ` +
          `Run task:disableHostChainOnGatewayConfig --chain-id ${chainId} first.`,
      );
    }

    console.log(`Removing host chain ${chainId} from GatewayConfig contract:`, proxyAddress, "\n");
    const tx = await gatewayConfig.removeHostChain(chainId);
    await tx.wait();

    console.log(`Emitted RemoveHostChain(${chainId}) (tx: ${tx.hash})`);
    console.log("Host chain removed!");
  });
