import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Computed as `keccak256("MINTER_ROLE")`. Matches both the real ZamaERC20 and the ERC20Mock.
export const MINTER_ROLE = '0x9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6';

// Minimal AccessControl ABI so this task works against any AccessControl-based ERC20 without
// depending on a specific compiled artifact (real ZamaERC20 or the testnet ERC20Mock).
const ACCESS_CONTROL_ABI = ['function grantRole(bytes32 role, address account) external'];

// Grant the ZAMA token's MINTER_ROLE to a given ProtocolStaking proxy, using the deployer account.
// The deployer must currently hold DEFAULT_ADMIN_ROLE on the token — otherwise the grant reverts.
async function grantMinterRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
  const { ethers, network, getNamedAccounts } = hre;

  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  const zamaTokenAddress = getRequiredEnvVar('ZAMA_TOKEN_ADDRESS');
  const token = new ethers.Contract(zamaTokenAddress, ACCESS_CONTROL_ABI, deployerSigner);

  const tx = await token.grantRole(MINTER_ROLE, protocolStakingProxyAddress);
  await tx.wait();

  console.log(
    [
      `🔑 Granted MINTER_ROLE on ZAMA token:`,
      `  - Token address: ${zamaTokenAddress}`,
      `  - New role holder (ProtocolStaking): ${protocolStakingProxyAddress}`,
      `  - Granted by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );
}

// Grant the ZAMA token's MINTER_ROLE to both ProtocolStaking proxies so that claimRewards can mint
// rewards. On production this is normally executed by the DAO via multisig; on testnet the deployer
// (which holds DEFAULT_ADMIN_ROLE on the mock) can call this task directly.
// Example usage:
// npx hardhat task:grantZamaTokenMinterRoleToProtocolStaking --network hoodi
task('task:grantZamaTokenMinterRoleToProtocolStaking').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Granting ZAMA token MINTER_ROLE to both ProtocolStaking contracts...\n');

  const coproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);
  await grantMinterRole(coproProxyAddress, hre);

  const kmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  await grantMinterRole(kmsProxyAddress, hre);

  console.log('✅ MINTER_ROLE granted to both ProtocolStaking contracts\n');
});
