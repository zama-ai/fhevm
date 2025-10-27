import { task, types } from 'hardhat/config'

import { logExplorerLink } from '../utils'

import { ROLE_TASK_SUFFIX, RoleKey, resolveRoleValue, resolveZamaRoleContext } from './helpers'

const ROLE_KEYS: RoleKey[] = ['MINTER_ROLE', 'PAUSING_MINTER_ROLE', 'DEFAULT_ADMIN_ROLE']

for (const role of ROLE_KEYS) {
    const suffix = ROLE_TASK_SUFFIX[role]
    task(`zama:erc20:grant:${suffix}`, `Grant ${role} to the provided address`)
        .addParam('address', 'Address to grant the role to', undefined, types.string)
        .setAction(async ({ address }, hre) => {
            if (!hre.ethers.utils.isAddress(address)) {
                throw new Error(`The provided address is not a valid EVM address: ${address}`)
            }

            const { signer, zamaErc20, deploymentAddress } = await resolveZamaRoleContext(hre)
            const roleValue = await resolveRoleValue(zamaErc20, role)

            const alreadyHasRole = await zamaErc20.hasRole(roleValue, address)
            if (alreadyHasRole) {
                console.log(`Address ${address} already has ${role} on contract ${deploymentAddress}`)
                return
            }

            console.log(
                `Granting ${role} (${roleValue}) to ${address} on contract ${deploymentAddress} using signer ${signer.address}`
            )

            const tx = await zamaErc20.grantRole(roleValue, address)
            console.log(`Transaction submitted: ${tx.hash}`)

            const receipt = await tx.wait()
            console.log(`Role granted in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

            await logExplorerLink(hre, tx.hash)
        })
}
