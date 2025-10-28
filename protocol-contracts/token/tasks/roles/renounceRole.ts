import { task } from 'hardhat/config'

import { resolveContext } from '../utils/contractContext'
import { logExplorerLink } from '../utils/lz'

import { ROLE_TASK_SUFFIX, RoleKey, resolveRoleValue } from './helpers'

const ROLE_KEYS: RoleKey[] = ['MINTER_ROLE', 'PAUSING_MINTER_ROLE', 'DEFAULT_ADMIN_ROLE']

for (const role of ROLE_KEYS) {
    const suffix = ROLE_TASK_SUFFIX[role]
    task(`zama:erc20:renounce:${suffix}`, `Renounce ${role} for the connected signer`).setAction(async (_, hre) => {
        const { signer, contract, deploymentAddress } = await resolveContext('ZamaERC20', hre)
        const roleValue = await resolveRoleValue(contract, role)

        const roleAdmin = await contract.getRoleAdmin(roleValue)
        const hasRoleAdmin = await contract.hasRole(roleAdmin, signer.address)
        if (!hasRoleAdmin) {
            throw new Error(
                `The deployer account ${signer.address} does not have the required admin role of ${role} for the ZamaERC20 contract ${deploymentAddress}`
            )
        }

        const hasRole = await contract.hasRole(roleValue, signer.address)
        if (!hasRole) {
            console.log(`Signer ${signer.address} does not have ${role} on contract ${deploymentAddress}`)
            return
        }

        console.log(`Renouncing ${role} (${roleValue}) from signer ${signer.address} on contract ${deploymentAddress}`)

        const tx = await contract.renounceRole(roleValue, signer.address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Role renounced in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
}
