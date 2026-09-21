import { task, types } from 'hardhat/config'

import { resolveContext } from '../utils/contractContext'
import { logExplorerLink } from '../utils/lz'

import { ROLE_TASK_SUFFIX, RoleKey, resolveRoleValue } from './helpers'

const ROLE_KEYS: RoleKey[] = ['MINTER_ROLE', 'MINTING_PAUSER_ROLE', 'DEFAULT_ADMIN_ROLE']

for (const role of ROLE_KEYS) {
    const suffix = ROLE_TASK_SUFFIX[role]
    task(`zama:erc20:renounce:${suffix}`, `Renounce ${role} for the connected signer`)
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
        .setAction(async ({ fromDeployment, contractAddress }, hre) => {
            const { signer, contract, deploymentAddress } = await resolveContext(
                'ZamaERC20',
                hre,
                fromDeployment,
                contractAddress
            )
            const roleValue = await resolveRoleValue(contract, role)

            const hasRole = await contract.hasRole(roleValue, signer.address)
            if (!hasRole) {
                throw new Error(`Signer ${signer.address} does not have ${role} on contract ${deploymentAddress}`)
            }

            console.log(
                `Renouncing ${role} (${roleValue}) from signer ${signer.address} on contract ${deploymentAddress}`
            )

            const tx = await contract.renounceRole(roleValue, signer.address)
            console.log(`Transaction submitted: ${tx.hash}`)

            const receipt = await tx.wait()
            console.log(`Role renounced in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

            await logExplorerLink(hre, tx.hash)
        })
}
