import { TransactionBuilder, publicKey as umiPublicKey } from '@metaplex-foundation/umi'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { EndpointProgram } from '@layerzerolabs/lz-solana-sdk-v2/umi'
import { addressToBytes32 } from '@layerzerolabs/lz-v2-utilities'

import { deriveConnection, getExplorerTxLink } from '../index'

import { resolvePayloadHashBytes } from './endpointUtils'

interface BurnTaskArgs {
    eid: EndpointId // The endpoint ID for the Solana network.
    sender: string // The sender address (hex format).
    receiver: string // The receiver address (base58 format).
    srcEid: number // The source endpoint ID.
    nonce: string // The nonce.
    payloadHash?: string // The payload hash (hex format).
    guid?: string // The GUID (hex string, 32 bytes)
    message?: string // The message payload (hex string)
}

// Example: pnpm hardhat lz:oft:solana:burn --eid 40168 --sender <SENDER_OAPP> --receiver <RECEIVER_OAPP> --src-eid 40161 --nonce <NONCE> --payload-hash <PAYLOAD_HASH>
//    or: pnpm hardhat lz:oft:solana:burn --eid 40168 --sender <SENDER_OAPP> --receiver <RECEIVER_OAPP> --src-eid 40161 --nonce <NONCE> --guid <GUID> --message <MESSAGE>
// Note: either provide payloadHash OR (guid + message). No overlap or partials.
// Note: for GUID and message, you can refer to layerzeroscan.com and search via the source transaction hash to get the values
// Note: to retrieve the payload hash, view/curl https://scan-testnet.layerzero-api.com/v1/messages/tx/<SOURCE_TX_HASH> and search for 'payloadHash'

task('lz:oft:solana:burn', 'Burn a nonce on Solana')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168) eid', undefined, devtoolsTypes.eid)
    .addParam('sender', 'The sender address (hex format)', undefined, devtoolsTypes.string)
    .addParam('receiver', 'The receiver address (base58 format)', undefined, devtoolsTypes.string)
    .addParam('srcEid', 'The source endpoint ID', undefined, devtoolsTypes.int)
    .addParam('nonce', 'The nonce', undefined, devtoolsTypes.string)
    .addOptionalParam('payloadHash', 'The payload hash (hex format)', undefined, devtoolsTypes.string)
    .addOptionalParam('guid', 'The GUID (hex string, 32 bytes)', undefined, devtoolsTypes.string)
    .addOptionalParam('message', 'The message payload (hex string)', undefined, devtoolsTypes.string)
    .setAction(async ({ eid, sender, receiver, srcEid, nonce, payloadHash, guid, message }: BurnTaskArgs) => {
        const { umi, umiWalletSigner } = await deriveConnection(eid)
        const endpoint = new EndpointProgram.Endpoint(EndpointProgram.ENDPOINT_PROGRAM_ID)

        // Validate inputs and resolve payload hash bytes
        const payloadHashBytes = resolvePayloadHashBytes(payloadHash, guid, message)
        // Convert sender from hex to bytes32
        const senderBytes = addressToBytes32(sender)
        if (senderBytes.length !== 32) {
            throw new Error('Sender must be 32 bytes (64 hex characters)')
        }

        // Convert receiver to Umi PublicKey
        const receiverUmiPublicKey = umiPublicKey(receiver)

        // payloadHashBytes already validated and resolved

        const instruction = endpoint.oAppBurnNonce(umiWalletSigner, {
            nonce: BigInt(nonce),
            receiver: receiverUmiPublicKey,
            sender: new Uint8Array(senderBytes),
            srcEid,
            payloadHash: new Uint8Array(payloadHashBytes),
        })

        const builder = new TransactionBuilder([instruction])
        const { signature } = await builder.sendAndConfirm(umi)

        console.log(
            `Burn transaction successful! View here: ${getExplorerTxLink(
                bs58.encode(signature),
                eid === EndpointId.SOLANA_V2_TESTNET
            )}`
        )
    })
