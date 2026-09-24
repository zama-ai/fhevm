import { arrayify, hexlify } from '@ethersproject/bytes'
import { Umi } from '@metaplex-foundation/umi'
import { fromWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { Connection, PublicKey } from '@solana/web3.js'

import { EndpointPDADeriver } from '@layerzerolabs/lz-solana-sdk-v2'
import { EndpointProgram } from '@layerzerolabs/lz-solana-sdk-v2/umi'
import { keccak256 } from '@layerzerolabs/lz-v2-utilities'

export async function getInboundNonce(
    umi: Umi,
    connection: Connection,
    receiver: PublicKey,
    srcEid: number,
    senderNormalized: Uint8Array<ArrayBufferLike>
): Promise<bigint> {
    // we use EndpointPDADeriver + getAccountInfo  so that we can print the expected Nonce PDA if it's not found
    const epDeriver = new EndpointPDADeriver(new PublicKey(EndpointProgram.ENDPOINT_PROGRAM_ID))
    const [nonceAccount] = epDeriver.nonce(receiver, srcEid, senderNormalized)
    const accountInfo = await connection.getAccountInfo(nonceAccount)
    if (!accountInfo) {
        throw new Error(`Nonce account not found at address ${nonceAccount.toBase58()}`)
    }
    const nonceAccountInfo = await EndpointProgram.accounts.fetchNonce(umi, fromWeb3JsPublicKey(nonceAccount))
    const inboundNonce = nonceAccountInfo.inboundNonce
    return inboundNonce
}

export function generatePayloadHash(guid: string, message: string): string {
    // Convert to bytes
    const guidBytes = arrayify(guid)
    const messageBytes = arrayify(message)

    // Concatenate guid and message bytes (equivalent to Solana's hashv(&[&guid[..], &message[..]]))
    const payloadBytes = new Uint8Array(guidBytes.length + messageBytes.length)
    payloadBytes.set(guidBytes, 0)
    payloadBytes.set(messageBytes, guidBytes.length)

    // Hash using keccak256 (equivalent to Solana's hashv)
    return keccak256(hexlify(payloadBytes))
}

/**
 * Validate inputs and resolve payload hash bytes from either:
 * - payloadHash (hex string), or
 * - guid + message (both hex strings)
 */
export function resolvePayloadHashBytes(payloadHash?: string, guid?: string, message?: string): Uint8Array {
    const hasPayloadHash = typeof payloadHash === 'string' && payloadHash.length > 0
    const hasGuid = typeof guid === 'string' && guid.length > 0
    const hasMessage = typeof message === 'string' && message.length > 0

    if ((hasGuid || hasMessage) && hasPayloadHash) {
        throw new Error('Provide either payloadHash OR guid+message, not both')
    }
    if (hasGuid !== hasMessage) {
        throw new Error('Both guid and message are required together')
    }
    if (!hasPayloadHash && !(hasGuid && hasMessage)) {
        throw new Error('Provide either payloadHash or guid+message')
    }

    const payloadHashHex = hasPayloadHash
        ? (payloadHash as string)
        : generatePayloadHash(guid as string, message as string)
    const bytes = arrayify(payloadHashHex)
    if (bytes.length !== 32) {
        throw new Error('Payload hash must be 32 bytes (64 hex characters)')
    }
    return bytes
}
