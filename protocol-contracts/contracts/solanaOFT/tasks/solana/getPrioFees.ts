import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { getPrioritizationFees } from '@layerzerolabs/devtools-solana'
import { EndpointId } from '@layerzerolabs/lz-definitions'

import { deriveConnection } from './index'

interface GetPrioFeesTaskArgs {
    /**
     * The endpoint ID for the Solana network.
     */
    eid: EndpointId
    /**
     * The program ID or account address that will be written to.
     */
    address: string
}

task('lz:solana:get-priority-fees', 'Fetches prioritization fees from the Solana network')
    .addParam('eid', 'The endpoint ID for the Solana network', undefined, devtoolsTypes.eid)
    .addOptionalParam(
        'address',
        'The address (program ID or account address) that will be written to',
        undefined,
        devtoolsTypes.string
    )
    .setAction(async ({ eid, address }: GetPrioFeesTaskArgs) => {
        const { connection } = await deriveConnection(eid, true)
        const fees = await getPrioritizationFees(connection, address)
        console.log('Prioritization Fees:', fees)
    })
