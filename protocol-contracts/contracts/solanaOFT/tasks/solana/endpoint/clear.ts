import { TransactionBuilder, publicKey as umiPublicKey } from '@metaplex-foundation/umi'
import { toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { normalizePeer } from '@layerzerolabs/devtools'
import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { EndpointProgram } from '@layerzerolabs/lz-solana-sdk-v2/umi'

import { deriveConnection, getExplorerTxLink } from '../index'

import { getInboundNonce } from './endpointUtils'

interface ClearTaskArgs {
    eid: EndpointId // The endpoint ID for the Solana network.
    sender: string // The sender address (hex format).
    receiver: string // The receiver address (base58 format).
    srcEid: number // The source endpoint ID.
    nonce: string // The nonce.
    guid: string // The GUID (hex string, 32 bytes)
    message: string // The message payload (hex string)
}

// Example: pnpm hardhat lz:oft:solana:clear --eid 40168 --sender <SENDER_OAPP> --receiver <RECEIVER_OAPP> --src-eid 40161 --nonce <NONCE> --guid <GUID_HEX_32B> --message <MESSAGE_HEX>
// Note: for GUID and message, you can refer to layerzeroscan.com and search via the source transaction hash to get the values
task('lz:oft:solana:clear', 'Clear a payload on Solana')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168) eid', undefined, devtoolsTypes.eid)
    .addParam('sender', 'The sender address (hex format)', undefined, devtoolsTypes.string)
    .addParam('receiver', 'The receiver address (base58 format)', undefined, devtoolsTypes.string)
    .addParam('srcEid', 'The source endpoint ID', undefined, devtoolsTypes.int)
    .addParam('nonce', 'The nonce', undefined, devtoolsTypes.string)
    .addParam('guid', 'The GUID (hex string, 32 bytes)', undefined, devtoolsTypes.string)
    .addParam('message', 'The message payload (hex string)', undefined, devtoolsTypes.string)
    .setAction(async ({ eid, sender, receiver, srcEid, nonce, guid, message }: ClearTaskArgs) => {
        const { umi, connection, umiWalletSigner } = await deriveConnection(eid)
        const endpoint = new EndpointProgram.Endpoint(EndpointProgram.ENDPOINT_PROGRAM_ID)

        // Convert sender from hex to bytes32
        const senderNormalized = normalizePeer(sender, srcEid)

        // Convert receiver to Umi PublicKey
        const receiverUmiPublicKey = umiPublicKey(receiver)

        const inboundNonce = await getInboundNonce(
            umi,
            connection,
            toWeb3JsPublicKey(receiverUmiPublicKey),
            srcEid,
            new Uint8Array(senderNormalized)
        )
        console.log('inboundNonce: ', inboundNonce.toString())

        // BOF nonce value validation
        // For clear, nonce must be equal to or less than inboundNonce
        if (BigInt(nonce) > inboundNonce) {
            throw new Error('Nonce must be equal to or less than inboundNonce')
        }
        // EOF nonce value validation

        // Convert GUID from hex to bytes
        const guidBytes = Buffer.from(guid.replace('0x', ''), 'hex')
        if (guidBytes.length !== 32) {
            throw new Error('GUID must be 32 bytes (64 hex characters)')
        }
        // Convert message from hex to bytes
        const messageBytes = Buffer.from(message.replace('0x', ''), 'hex')

        // Derive PDAs
        const [endpointPda] = endpoint.pda.setting()
        const [noncePda] = endpoint.pda.nonce(receiverUmiPublicKey, srcEid, new Uint8Array(senderNormalized))
        const [oappRegistryPda] = endpoint.pda.oappRegistry(receiverUmiPublicKey)
        const [payloadHashPda] = endpoint.pda.payloadHash(
            receiverUmiPublicKey,
            srcEid,
            new Uint8Array(senderNormalized),
            Number(nonce)
        )

        // Check payload hash account existence; only init if missing
        const payloadAccountInfo = await umi.rpc.getAccount(payloadHashPda)
        const ixns = [] as ReturnType<typeof EndpointProgram.instructions.clear>['items']

        if (!payloadAccountInfo.exists) {
            const initVerifyIxn = endpoint.initVerify(umiWalletSigner, {
                srcEid,
                sender: senderNormalized,
                receiver: receiverUmiPublicKey,
                nonce: BigInt(nonce),
            })
            ixns.push(initVerifyIxn)
        }

        const clearIxn = EndpointProgram.instructions.clear(
            { programs: endpoint.programRepo },
            {
                signer: umiWalletSigner,
                oappRegistry: oappRegistryPda,
                nonce: noncePda,
                payloadHash: payloadHashPda,
                endpoint: endpointPda,
                eventAuthority: endpoint.eventAuthority,
                program: endpoint.programId,
            },
            {
                receiver: receiverUmiPublicKey,
                srcEid,
                sender: new Uint8Array(senderNormalized),
                nonce: BigInt(nonce),
                guid: new Uint8Array(guidBytes),
                message: new Uint8Array(messageBytes),
            }
        ).items[0]

        const builder = new TransactionBuilder([...ixns, clearIxn])
        const { signature } = await builder.sendAndConfirm(umi)

        console.log(
            `Clear transaction successful! View here: ${getExplorerTxLink(
                bs58.encode(signature),
                eid === EndpointId.SOLANA_V2_TESTNET
            )}`
        )
    })
