import { task, types } from 'hardhat/config'

import { resolveContext } from '../utils/contractContext'
import { logExplorerLink } from '../utils/lz'

import { ROLE_TASK_SUFFIX, RoleKey, resolveRoleValue } from './helpers'

const ROLE_KEYS: RoleKey[] = ['MINTER_ROLE', 'PAUSING_MINTER_ROLE', 'DEFAULT_ADMIN_ROLE']

for (const role of ROLE_KEYS) {
    const suffix = ROLE_TASK_SUFFIX[role]
    task(`zama:erc20:grant:${suffix}`, `Grant ${role} to the provided address`)
        .addParam('address', 'Address to grant the role to', undefined, types.string)
        .addOptionalParam(
            'fromDeployment',
            'Fetch the address of the ZamaERC20 contract from the existing deployments for the selected network.',
            false,
            types.boolean
        )
        .addOptionalParam(
            'contractAddress',
            'Address of the ZamaERC20 contract to interact with. It not set, it fallback on ZAMAERC20_CONTRACT_ADDRESS env variable.',
            undefined,
            types.string
        )
        .setAction(async ({ address, fromDeployment, contractAddress }, hre) => {
            if (!hre.ethers.utils.isAddress(address)) {
                throw new Error(`The provided address is not a valid EVM address: ${address}`)
            }

            const { signer, contract, deploymentAddress } = await resolveContext(
                'ZamaERC20',
                hre,
                fromDeployment,
                contractAddress
            )
            const roleValue = await resolveRoleValue(contract, role)

            const roleAdmin = await contract.getRoleAdmin(roleValue)
            const hasRoleAdmin = await contract.hasRole(roleAdmin, signer.address)
            if (!hasRoleAdmin) {
                throw new Error(
                    `The deployer account ${signer.address} does not have the required admin role of ${role} for the ZamaERC20 contract ${deploymentAddress}`
                )
            }

            const alreadyHasRole = await contract.hasRole(roleValue, address)
            if (alreadyHasRole) {
                throw new Error(`Address ${address} already has ${role} on contract ${deploymentAddress}`)
            }

            console.log(
                `Granting ${role} (${roleValue}) to ${address} on contract ${deploymentAddress} using signer ${signer.address}`
            )

            const tx = await contract.grantRole(roleValue, address)
            console.log(`Transaction submitted: ${tx.hash}`)

            const receipt = await tx.wait()
            console.log(`Role granted in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

            await logExplorerLink(hre, tx.hash)
        })
}
