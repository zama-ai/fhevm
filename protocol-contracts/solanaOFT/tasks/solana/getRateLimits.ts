import { mplToolbox } from '@metaplex-foundation/mpl-toolbox'
import { publicKey } from '@metaplex-foundation/umi'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { task } from 'hardhat/config'

import { types } from '@layerzerolabs/devtools-evm-hardhat'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { OftPDA, accounts } from '@layerzerolabs/oft-v2-solana-sdk'

import { createSolanaConnectionFactory } from '../common/utils'

interface Args {
    mint: string
    eid: EndpointId
    dstEid: EndpointId
    programId: string
    oftStore: string
}

task('lz:oft:solana:get-rate-limits', 'Gets the Solana inbound / outbound rate limits')
    .addParam('mint', 'The OFT token mint public key')
    .addParam('programId', 'The OFT Program id')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168)', undefined, types.eid)
    .addParam('dstEid', 'The destination endpoint ID', undefined, types.eid)
    .addParam('oftStore', 'The OFTStore account')
    .setAction(async (taskArgs: Args, _) => {
        const connectionFactory = createSolanaConnectionFactory()
        const connection = await connectionFactory(taskArgs.eid)
        const umi = createUmi(connection.rpcEndpoint).use(mplToolbox())

        const [peer] = new OftPDA(publicKey(taskArgs.programId)).peer(publicKey(taskArgs.oftStore), taskArgs.dstEid)
        const peerInfo = await accounts.fetchPeerConfig({ rpc: umi.rpc }, peer)
        console.log(`Peer info between ${taskArgs.eid} and ${taskArgs.dstEid}`)
        console.dir({ peerInfo }, { depth: null })
    })
