// Logic flattened from LayerZero's devtools-evm-hardhat, used in their lz:deploy hardhat task.
// References can be found there:
// - https://github.com/LayerZero-Labs/devtools/blob/77ba15585252d58e1f613d69bef5013b15be0240/packages/devtools-evm-hardhat/src/runtime.ts#L119
// - https://github.com/LayerZero-Labs/devtools/blob/77ba15585252d58e1f613d69bef5013b15be0240/packages/devtools-evm-hardhat/src/tasks/deploy.ts#L195
import { HardhatContext } from "hardhat/internal/context";
import { Environment as HardhatRuntimeEnvironmentImplementation } from "hardhat/internal/core/runtime-environment";
import { HardhatRuntimeEnvironment } from "hardhat/types";

/**
 * Generic type for a hybrid (sync / async) factory
 * that generates an instance of `TOutput` based on arguments of type `TInput`
 *
 * `TInput` represents the list of all function arguments that need to be passed to the factory:
 *
 * ```typescript
 * const mySyncFactory: Factory<[number, boolean], string> = (num: number, bool: boolean): string => "hello"
 *
 * const mySyncFactory: Factory<[], string> = async () => "hello"
 * ```
 *
 * The hybrid aspect just makes it easier for implementers - if the logic is synchronous,
 * this type will not force any extra `async`.
 */
export type Factory<TInput extends unknown[], TOutput> = (...input: TInput) => TOutput | Promise<TOutput>;

/**
 * Helper type for when we need to grab something asynchronously by the network name
 */
export type GetByNetwork<TValue> = Factory<[networkName: string], TValue>;

/**
 * Returns the default hardhat context for the project, i.e.
 * the context that the project has been setup with.
 *
 * Throws if there is no context.
 *
 * @returns {HardhatContext}
 */
const getDefaultContext = (): HardhatContext => {
  // Context is registered globally as a singleton and can be accessed
  // using the static methods of the HardhatContext class
  //
  // In our case we require the context to exist, the other option would be
  // to create it and set it up - see packages/hardhat-core/src/register.ts for an example setup
  try {
    return HardhatContext.getHardhatContext();
  } catch (error: unknown) {
    throw new Error(`Could not get Hardhat context: ${error}`);
  }
};

/**
 * Returns the default `HardhatRuntimeEnvironment` (`hre`) for the project.
 *
 * Throws if there is no `HardhatRuntimeEnvironment`.
 *
 * @returns {HardhatRuntimeEnvironment}
 */
const getDefaultRuntimeEnvironment = (): HardhatRuntimeEnvironment => {
  // The first step is to get the hardhat context
  const context = getDefaultContext();

  // We require the hardhat environment to already exist
  //
  // Again, we could create it but that means we'd need to duplicate the bootstrap code
  // that hardhat does when setting up the environment
  try {
    return context.getHardhatRuntimeEnvironment();
  } catch (error: unknown) {
    throw new Error(`Could not get Hardhat Runtime Environment: ${error}`);
  }
};

/**
 * Creates a clone of the HardhatRuntimeEnvironment for a particular network
 *
 * ```typescript
 * const env = getHreByNetworkName("bsc-testnet");
 *
 * // All the ususal properties are present
 * env.deployments.get("MyContract")
 * ```
 *
 * @returns {Promise<HardhatRuntimeEnvironment>}
 */
export const getHreByNetworkName: GetByNetwork<HardhatRuntimeEnvironment> = async (
  networkName,
): Promise<HardhatRuntimeEnvironment> => {
  const context = getDefaultContext();
  const environment = getDefaultRuntimeEnvironment();

  try {
    // The last step is to create a duplicate environment that mimics the original one
    // with one crucial difference - the network setup
    return new HardhatRuntimeEnvironmentImplementation(
      environment.config,
      {
        ...environment.hardhatArguments,
        network: networkName,
      },
      environment.tasks,
      environment.scopes,
      context.environmentExtenders,
      environment.userConfig,
      context.providerExtenders,
      // This is a bit annoying - the environmentExtenders are not stronly typed
      // so TypeScript complains that the properties required by HardhatRuntimeEnvironment
      // are not present on HardhatRuntimeEnvironmentImplementation
    ) as unknown as HardhatRuntimeEnvironment;
  } catch (error: unknown) {
    throw new Error(`Could not setup Hardhat Runtime Environment: ${error}`);
  }
};
