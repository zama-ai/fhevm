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

interface SkipTaskArgs {
    eid: EndpointId // The endpoint ID for the Solana network.
    sender: string // The sender address (hex format).
    receiver: string // The receiver address (base58 format).
    srcEid: number // The source endpoint ID.
    nonce: string // The nonce.
}

// Example: pnpm hardhat lz:oft:solana:skip --eid 40168 --sender <SENDER_OAPP> --receiver <RECEIVER_OAPP> --src-eid 40161 --nonce <NONCE>
task('lz:oft:solana:skip', 'Skip a message on Solana')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168) eid', undefined, devtoolsTypes.eid)
    .addParam('sender', 'The sender address (hex format)', undefined, devtoolsTypes.string)
    .addParam('receiver', 'The receiver address (base58 format)', undefined, devtoolsTypes.string)
    .addParam('srcEid', 'The source endpoint ID', undefined, devtoolsTypes.int)
    .addParam('nonce', 'The nonce', undefined, devtoolsTypes.string)
    .setAction(async ({ eid, sender, receiver, srcEid, nonce }: SkipTaskArgs) => {
        const { umi, connection, umiWalletSigner } = await deriveConnection(eid)
        const endpoint = new EndpointProgram.Endpoint(EndpointProgram.ENDPOINT_PROGRAM_ID)

        const senderNormalized = normalizePeer(sender, srcEid)
        const receiverUmiPublicKey = umiPublicKey(receiver) // Convert receiver from string to Umi PublicKey

        const inboundNonce = await getInboundNonce(
            umi,
            connection,
            toWeb3JsPublicKey(receiverUmiPublicKey),
            srcEid,
            senderNormalized
        )
        // print inboundNonce
        console.log('inboundNonce: ', inboundNonce.toString())

        // BOF nonce value validation
        // throw if nonce is not greater than inboundNonce
        if (BigInt(nonce) <= inboundNonce) {
            throw new Error('Nonce must be greater than inboundNonce')
        }
        // throw if nonce is greather than sliding window
        const PENDING_INBOUND_NONCE_MAX_LEN = BigInt(256)
        if (BigInt(nonce) > inboundNonce + PENDING_INBOUND_NONCE_MAX_LEN) {
            throw new Error('Nonce must not be greater than inboundNonce + sliding window range (256)')
        }
        // EOF nonce value validation

        const initVerifyIxn = endpoint.initVerify(umiWalletSigner, {
            srcEid,
            sender: senderNormalized,
            receiver: receiverUmiPublicKey,
            nonce: BigInt(nonce),
        })

        const skipIxn = endpoint.skip(umiWalletSigner, {
            sender: senderNormalized,
            receiver: receiverUmiPublicKey,
            srcEid,
            nonce: BigInt(nonce),
        })

        const builder = new TransactionBuilder([initVerifyIxn, skipIxn])
        const { signature } = await builder.sendAndConfirm(umi)

        console.log(
            `Skip transaction successful! View here: ${getExplorerTxLink(
                bs58.encode(signature),
                eid === EndpointId.SOLANA_V2_TESTNET
            )}`
        )
    })
