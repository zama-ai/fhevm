// The network connection shared by every test, and what HH2 tests imported from 'hardhat'.
//
// `network.getOrCreate()` returns the connection task:prepareTestContracts deployed the host contracts on (it runs
// before the tests, in the same process; see the `test mocha` override in hardhat.config.ts).
import hre from 'hardhat';

export const connection = await hre.network.getOrCreate();

/** ethers (the library) with the hardhat-ethers helpers, bound to the shared connection. HH2: `ethers` from 'hardhat'. */
export const { ethers } = connection;

/** HH2: `network.name`. */
export const networkName = connection.networkName;

/** `true` on the in-process network, where the coprocessor and the relayer are mocked. HH2: `network.name === 'hardhat'`. */
export const isInProcessNetwork = connection.networkConfig.type === 'edr-simulated';

/** HH2: `network.config.chainId`. */
export const chainId: number | undefined = connection.networkConfig.chainId;
