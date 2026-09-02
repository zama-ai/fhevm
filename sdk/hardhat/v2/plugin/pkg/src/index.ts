// The FHEVM hardhat plugin depends on '@nomicfoundation/hardhat-ethers'
// It must be imported in the plugin root to make sure the rest of the code
// can access the hre.ethers property.
import '@nomicfoundation/hardhat-ethers';
import { extendConfig, extendEnvironment, extendProvider } from 'hardhat/config';

/**
 * HH Plugin Config
 */
import { configExtender } from './internal/ConfigExtender';
import { envExtender } from './internal/EnvironmentExtender';
import { providerExtender } from './internal/ProviderExtender';
import './tasks/builtin-tasks';
/**
 * Tasks
 */
import './tasks/fhevm';
import './type-extensions';

extendConfig(configExtender);

extendEnvironment(envExtender);

extendProvider(providerExtender);

export * from './types';
