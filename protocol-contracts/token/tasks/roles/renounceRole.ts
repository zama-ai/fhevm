import { task } from 'hardhat/config'

import { logExplorerLink } from '../utils'

import { ROLE_TASK_SUFFIX, RoleKey, resolveRoleValue, resolveZamaRoleContext } from './helpers'

const ROLE_KEYS: RoleKey[] = ['MINTER_ROLE', 'PAUSING_MINTER_ROLE', 'DEFAULT_ADMIN_ROLE']

for (const role of ROLE_KEYS) {
    const suffix = ROLE_TASK_SUFFIX[role]
    task(`zama:erc20:renounce:${suffix}`, `Renounce ${role} for the connected signer`).setAction(async (_, hre) => {
        const { signer, zamaErc20, deploymentAddress } = await resolveZamaRoleContext(hre)
        const roleValue = await resolveRoleValue(zamaErc20, role)

        const signerAddress = signer.address
        const hasRole = await zamaErc20.hasRole(roleValue, signerAddress)
        if (!hasRole) {
            console.log(`Signer ${signerAddress} does not have ${role} on contract ${deploymentAddress}`)
            return
        }

        console.log(`Renouncing ${role} (${roleValue}) from signer ${signerAddress} on contract ${deploymentAddress}`)

        const tx = await zamaErc20.renounceRole(roleValue, signerAddress)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Role renounced in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
}
