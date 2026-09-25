import { AccountMeta, publicKey, transactionBuilder } from '@metaplex-foundation/umi'
import { fromWeb3JsPublicKey, toWeb3JsKeypair, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { AuthorityType, TOKEN_PROGRAM_ID, createSetAuthorityInstruction, getMint } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { OftPDA } from '@layerzerolabs/oft-v2-solana-sdk'

import { checkMultisigSigners, createMintAuthorityMultisig } from './multisig'

import { TransactionType, addComputeUnitInstructions, deriveConnection, getExplorerTxLink } from './index'

interface SetAuthorityTaskArgs {
    /**
     * The endpoint ID for the Solana network.
     */
    eid: EndpointId

    /**
     * The escrow public key.
     */
    escrow: string

    /**
     * The token mint ID, for Mint-And-Burn-Adapter only.
     */
    mint: string

    /**
     * The program ID for the OFT program.
     */
    programId: string

    /**
     * The CSV list of additional minters.
     */
    additionalMinters?: string[]

    /**
     * The token program ID, for Mint-And-Burn-Adapter only.
     */
    tokenProgram: string

    /**
     * If you plan to have only the OFTStore and no additional minters.  This is not reversible, and will result in
     * losing the ability to mint new tokens for everything but the OFTStore.  You should really be intentional about
     * using this flag, as it is not reversible.
     */
    onlyOftStore: boolean

    computeUnitPriceScaleFactor: number
}

/**
 * Derive the OFT Store account for a given program and escrow.
 * @param programId {string}
 * @param escrow {string}
 */
const getOftStore = (programId: string, escrow: string) => {
    const oftDeriver = new OftPDA(publicKey(programId))
    const escrowPK = publicKey(escrow)
    const [oftStorePda] = oftDeriver.oftStore(escrowPK)
    return oftStorePda
}

/**
 * Get the string representation of the authority type.
 * @param authorityType {AuthorityType}
 */
const getAuthorityTypeString = (authorityType: AuthorityType) => {
    switch (authorityType) {
        case AuthorityType.MintTokens:
            return 'MintTokens'
        case AuthorityType.FreezeAccount:
            return 'FreezeAccount'
        default:
            throw Error(`Unknown authority type: ${authorityType}`)
    }
}

// Define a Hardhat task for creating and setting a new Mint/Freeze Authority
// for OFT on Solana
// * Create SPL Multisig account for mint authority
// * Sanity check the new Multisig account
// * Set Mint Authority
// * Set Freeze Authority
// Note:  Only supports SPL Token Standard.
task('lz:oft:solana:setauthority', 'Create a new Mint Authority SPL multisig and set the mint/freeze authority')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168) eid', undefined, devtoolsTypes.eid)
    .addParam('mint', 'The Token Mint public key')
    .addParam('programId', 'The OFT Program id')
    .addParam('escrow', 'The OFT Escrow public key')
    .addParam('additionalMinters', 'Comma-separated list of additional minters', undefined, devtoolsTypes.csv, true)
    .addOptionalParam(
        'onlyOftStore',
        'If you plan to have only the OFTStore and no additional minters.  This is not reversible, and will result in losing the ability to mint new tokens by everything but the OFTStore.',
        false,
        devtoolsTypes.boolean
    )
    .addParam(
        'tokenProgram',
        'The Token Program public key (used for MABA only)',
        TOKEN_PROGRAM_ID.toBase58(),
        devtoolsTypes.string
    )
    .addParam('computeUnitPriceScaleFactor', 'The compute unit price scale factor', 4, devtoolsTypes.float, true)
    .setAction(
        async ({
            eid,
            escrow: escrowStr,
            mint: mintStr,
            programId: programIdStr,
            tokenProgram: tokenProgramStr,
            additionalMinters: additionalMintersAsStrings,
            onlyOftStore,
            computeUnitPriceScaleFactor,
        }: SetAuthorityTaskArgs) => {
            const { connection, umi, umiWalletKeyPair, umiWalletSigner } = await deriveConnection(eid)
            const oftStorePda = getOftStore(programIdStr, escrowStr)
            const tokenProgram = publicKey(tokenProgramStr)
            if (!additionalMintersAsStrings) {
                if (!onlyOftStore) {
                    throw new Error(
                        'If you want to proceed with only the OFTStore, please specify --only-oft-store true'
                    )
                }
                console.log(
                    'No additional minters specified.  This will result in only the OFTStore being able to mint new tokens.'
                )
            }
            const additionalMinters = additionalMintersAsStrings?.map((minter) => new PublicKey(minter)) ?? []
            const mint = new PublicKey(mintStr)
            const newMintAuthority = await createMintAuthorityMultisig(
                connection,
                umi,
                eid,
                umiWalletSigner,
                new PublicKey(oftStorePda.toString()),
                new PublicKey(tokenProgram.toString()),
                additionalMinters,
                computeUnitPriceScaleFactor
            )
            console.log(`New Mint Authority: ${newMintAuthority.toBase58()}`)
            const signers = await checkMultisigSigners(connection, newMintAuthority, [
                toWeb3JsPublicKey(oftStorePda),
                ...additionalMinters,
            ])
            console.log(`New Mint Authority Signers: ${signers.map((s) => s.toBase58()).join(', ')}`)
            for (const authorityType of [AuthorityType.MintTokens, AuthorityType.FreezeAccount]) {
                const mintAuthRet = await getMint(connection, mint, undefined, toWeb3JsPublicKey(tokenProgram))
                let currentAuthority
                if (authorityType == AuthorityType.MintTokens) {
                    if (!mintAuthRet.mintAuthority) {
                        throw new Error(`Mint ${mintStr} has no mint authority`)
                    }
                    currentAuthority = fromWeb3JsPublicKey(mintAuthRet.mintAuthority)
                } else {
                    if (!mintAuthRet.freezeAuthority) {
                        throw new Error(`Mint ${mintStr} has no freeze authority`)
                    }
                    currentAuthority = fromWeb3JsPublicKey(mintAuthRet.freezeAuthority)
                }
                if (authorityType == AuthorityType.FreezeAccount && !mintAuthRet.freezeAuthority) {
                    throw new Error(`Mint ${mintStr} has no freeze authority`)
                }
                console.log(`Current ${getAuthorityTypeString(authorityType)} Authority: ${currentAuthority}`)
                const ix = createSetAuthorityInstruction(
                    new PublicKey(mintStr),
                    toWeb3JsPublicKey(currentAuthority),
                    authorityType,
                    newMintAuthority,
                    [toWeb3JsKeypair(umiWalletKeyPair)]
                )
                const umiInstruction = {
                    programId: publicKey(ix.programId.toBase58()),
                    keys: ix.keys.map((key) => ({
                        pubkey: key.pubkey,
                        isSigner: key.isSigner,
                        isWritable: key.isWritable,
                    })) as unknown as AccountMeta[],
                    data: ix.data,
                }
                let txBuilder = transactionBuilder().add({
                    instruction: umiInstruction,
                    signers: [umiWalletSigner], // Include all required signers here
                    bytesCreatedOnChain: 0,
                })
                txBuilder = await addComputeUnitInstructions(
                    connection,
                    umi,
                    eid,
                    txBuilder,
                    umiWalletSigner,
                    computeUnitPriceScaleFactor,
                    TransactionType.SetAuthority
                )
                const { signature } = await txBuilder.sendAndConfirm(umi)
                console.log(
                    `SetAuthorityTx(${getAuthorityTypeString(authorityType)}): ${getExplorerTxLink(bs58.encode(signature), eid == EndpointId.SOLANA_V2_TESTNET)}`
                )
            }
        }
    )
