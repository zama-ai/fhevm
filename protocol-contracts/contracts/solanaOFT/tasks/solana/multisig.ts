import { createAccount } from '@metaplex-foundation/mpl-toolbox'
import {
    KeypairSigner,
    Umi,
    createSignerFromKeypair,
    transactionBuilder,
    publicKey as umiPublicKey,
} from '@metaplex-foundation/umi'
import { fromWeb3JsInstruction, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import {
    MULTISIG_SIZE,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    createInitializeMultisigInstruction,
} from '@solana/spl-token'
import { Connection, PublicKey } from '@solana/web3.js'
import bs58 from 'bs58'

import { assertAccountInitialized } from '@layerzerolabs/devtools-solana'
import { EndpointId } from '@layerzerolabs/lz-definitions'

import { TransactionType, addComputeUnitInstructions, getExplorerTxLink } from '.'

export async function createMultisig(
    connection: Connection,
    umi: Umi,
    eid: EndpointId,
    umiWalletSigner: KeypairSigner,
    signers: PublicKey[],
    m: number,
    keypair = umi.eddsa.generateKeypair(),
    programId = TOKEN_PROGRAM_ID,
    computeUnitPriceScaleFactor?: number
): Promise<PublicKey> {
    const initMultisigIx = createInitializeMultisigInstruction(
        toWeb3JsPublicKey(keypair.publicKey),
        signers,
        m,
        programId
    )

    let txBuilder = transactionBuilder()
        .add(
            createAccount(umi, {
                newAccount: createSignerFromKeypair(umi, keypair),
                lamports: await umi.rpc.getRent(MULTISIG_SIZE),
                space: MULTISIG_SIZE,
                programId: umiPublicKey(programId.toBase58()),
            })
        )
        .add({
            instruction: fromWeb3JsInstruction(initMultisigIx),
            signers: [],
            bytesCreatedOnChain: 0,
        })

    if (computeUnitPriceScaleFactor) {
        txBuilder = await addComputeUnitInstructions(
            connection,
            umi,
            eid,
            txBuilder,
            umiWalletSigner,
            computeUnitPriceScaleFactor,
            TransactionType.CreateMultisig
        )
    }

    const multisigPublicKey = toWeb3JsPublicKey(keypair.publicKey)

    const tx = await txBuilder.sendAndConfirm(umi)
    await assertAccountInitialized(connection, multisigPublicKey)
    const isTestnet = eid == EndpointId.SOLANA_V2_TESTNET
    console.log(`createMultisigTx: ${getExplorerTxLink(bs58.encode(tx.signature), isTestnet)}`)

    return multisigPublicKey
}

/**
 * Creates a (1/N) multisig account for use as the mint authority.
 * @param connection {Connection}
 * @param payer {Signer}
 * @param oftStorePda {PublicKey} will be included as a signer
 * @param tokenProgramId {PublicKey} defaults to SPL token program ID
 * @param additionalSigners {PublicKey[]} the additionalSigners for the multisig account
 */
export const createMintAuthorityMultisig = async (
    connection: Connection,
    umi: Umi,
    eid: EndpointId,
    umiWalletSigner: KeypairSigner,
    oftStorePda: PublicKey,
    tokenProgramId: PublicKey = TOKEN_PROGRAM_ID,
    additionalSigners: PublicKey[],
    computeUnitPriceScaleFactor: number
) => {
    return createMultisig(
        connection,
        umi,
        eid,
        umiWalletSigner,
        [oftStorePda, ...additionalSigners],
        1, // quorum 1/N
        undefined,
        tokenProgramId,
        computeUnitPriceScaleFactor
    )
}

/**
 * Decode the signers of a multisig account and check if the expected signers
 * are present and the quorum is 1/N.
 * @param connection {Connection}
 * @param multisigAddress {PublicKey}
 * @param expectedSigners {PublicKey[]} the expected signers
 */
export const checkMultisigSigners = async (
    connection: Connection,
    multisigAddress: PublicKey,
    expectedSigners: PublicKey[]
) => {
    const accountInfo = await assertAccountInitialized(connection, multisigAddress)

    if (!accountInfo.owner.equals(TOKEN_PROGRAM_ID) && !accountInfo.owner.equals(TOKEN_2022_PROGRAM_ID)) {
        throw new Error('Provided address is not an SPL Token multisig account')
    }

    // Multisig accounts have a specific layout based on the Multisig interface:
    const data = accountInfo.data

    // Extract the number of required signers (m) and total possible signers (n)
    const numRequiredSigners = data[0]
    const numTotalSigners = data[1]

    if (numRequiredSigners !== 1) {
        throw new Error('Multisig account must have 1 required signer')
    }

    // Initialize an array to hold the signers
    const signers: PublicKey[] = []

    // Extract each signer public key based on the Multisig interface
    const signerOffset = 3 // Offset to the first signer in the data
    const signerSize = 32 // Each signer address is 32 bytes

    for (let i = 0; i < numTotalSigners; i++) {
        const start = signerOffset + i * signerSize
        const end = start + signerSize
        const signerPublicKey = new PublicKey(data.slice(start, end))
        if (!signerPublicKey.equals(PublicKey.default)) {
            signers.push(signerPublicKey)
        }
    }

    for (const signer of expectedSigners) {
        if (!signers.find((s) => s.toBase58() == signer.toBase58())) {
            throw new Error(`Signer ${signer.toBase58()} not found in multisig account`)
        }
    }
    return signers
}
