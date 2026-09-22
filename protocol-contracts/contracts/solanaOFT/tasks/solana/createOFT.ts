import {
    CreateV1InstructionAccounts,
    CreateV1InstructionArgs,
    TokenStandard,
    createV1,
    mintV1,
} from '@metaplex-foundation/mpl-token-metadata'
import { AuthorityType, setAuthority } from '@metaplex-foundation/mpl-toolbox'
import {
    createNoopSigner,
    createSignerFromKeypair,
    percentAmount,
    publicKey,
    transactionBuilder,
} from '@metaplex-foundation/umi'
import { fromWeb3JsPublicKey, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { formatTokenAmount } from '@layerzerolabs/devtools'
import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { assertAccountInitialized, localDecimalsToMaxWholeTokens } from '@layerzerolabs/devtools-solana'
import { promptToContinue } from '@layerzerolabs/io-devtools'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { OFT_DECIMALS as DEFAULT_SHARED_DECIMALS, oft } from '@layerzerolabs/oft-v2-solana-sdk'

import { checkMultisigSigners, createMintAuthorityMultisig } from './multisig'

import {
    TransactionType,
    addComputeUnitInstructions,
    deriveConnection,
    deriveKeys,
    getExplorerTxLink,
    saveSolanaDeployment,
} from './index'

const DEFAULT_LOCAL_DECIMALS = 9

interface CreateOFTTaskArgs {
    /**
     * The initial supply to mint on solana.
     */
    amount: number

    /**
     * The endpoint ID for the Solana network.
     */
    eid: EndpointId

    /**
     * The number of decimal places to use for the token.
     */
    localDecimals: number

    /**
     * OFT shared decimals.
     */
    sharedDecimals: number

    /**
     * The optional token mint ID, for Mint-And-Burn-Adapter only.
     */
    mint?: string

    /**
     * The name of the token.
     */
    name: string

    /**
     * The program ID for the OFT program.
     */
    programId: string

    /**
     * The seller fee basis points for Metaplex's Token Metadata standard (not enforced on-chain). This is not related to OFT fees.
     */
    sellerFeeBasisPoints: number

    /**
     * The symbol of the token.
     */
    symbol: string

    /**
     * Whether the token metadata is mutable.
     */
    tokenMetadataIsMutable: boolean

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

    /**
     * The URI for the token metadata.
     */
    uri: string

    computeUnitPriceScaleFactor: number

    /**
     * The freeze authority address (only supported in onlyOftStore mode).
     */
    freezeAuthority?: string

    /**
     * Whether to continue without confirmation.
     */
    ci: boolean
}

// Define a Hardhat task for creating OFT on Solana
// * Create the SPL Multisig account for mint authority
// * Mint the new SPL Token
// * Initialize the OFT Store account
// * Set the mint authority to the multisig account. If not in only OFT Store mode, also set the freeze authority to the multisig account.
// Note:  Only supports SPL Token Standard.
task('lz:oft:solana:create', 'Mints new SPL Token and creates new OFT Store account')
    .addOptionalParam('amount', 'The initial supply to mint on solana', undefined, devtoolsTypes.int)
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168)', undefined, devtoolsTypes.eid)
    .addOptionalParam('localDecimals', 'Token local decimals (default=9)', DEFAULT_LOCAL_DECIMALS, devtoolsTypes.int)
    .addOptionalParam('sharedDecimals', 'OFT shared decimals (default=6)', DEFAULT_SHARED_DECIMALS, devtoolsTypes.int)
    .addParam('name', 'Token Name', 'Zama', devtoolsTypes.string)
    .addOptionalParam('mint', 'The Token mint public key (used for MABA only)', undefined, devtoolsTypes.string)
    .addParam('programId', 'The OFT Program id')
    .addParam('sellerFeeBasisPoints', 'Seller fee basis points', 0, devtoolsTypes.int) // Note: This is for Metaplex's Token Metadata standard (not enforced on-chain). This is not related to OFT fees.
    .addParam('symbol', 'Token Symbol', 'ZAMA', devtoolsTypes.string)
    .addParam('tokenMetadataIsMutable', 'Token metadata is mutable', true, devtoolsTypes.boolean)
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
    .addParam('uri', 'URI for token metadata', '', devtoolsTypes.string)
    .addParam(
        'freezeAuthority',
        'The Freeze Authority address (only in onlyOftStore mode)',
        '',
        devtoolsTypes.string,
        true
    )
    .addParam('computeUnitPriceScaleFactor', 'The compute unit price scale factor', 4, devtoolsTypes.float, true)
    .addFlag('ci', 'Continue without confirmation')
    .setAction(
        async ({
            amount,
            eid,
            localDecimals: decimals,
            sharedDecimals,
            mint: mintStr,
            name,
            programId: programIdStr,
            sellerFeeBasisPoints,
            symbol,
            tokenMetadataIsMutable: isMutable,
            additionalMinters: additionalMintersAsStrings,
            onlyOftStore,
            tokenProgram: tokenProgramStr,
            uri,
            freezeAuthority: freezeAuthorityStr,
            computeUnitPriceScaleFactor,
            ci,
        }: CreateOFTTaskArgs) => {
            const isMABA = !!mintStr // the difference between MABA and OFT Adapter is that MABA uses mint/burn mechanism whereas OFT Adapter uses lock/unlock mechanism
            if (tokenProgramStr !== TOKEN_PROGRAM_ID.toBase58() && !isMABA) {
                throw new Error('Non-Mint-And-Burn-Adapter does not support custom token programs')
            }
            if (isMABA && amount) {
                throw new Error('Mint-And-Burn-Adapter does not support minting tokens')
            }
            if (decimals < sharedDecimals) {
                throw new Error('Solana token local decimals must be greater than or equal to OFT shared decimals')
            }
            const tokenProgramId = publicKey(tokenProgramStr)
            const { connection, umi, umiWalletKeyPair, umiWalletSigner } = await deriveConnection(eid)
            const { programId, lockBox, escrowPK, oftStorePda, eddsa } = deriveKeys(programIdStr)

            const additionalMinters = additionalMintersAsStrings?.map((minter) => new PublicKey(minter)) ?? []

            // BOF: Validate combination of parameters
            if (!additionalMintersAsStrings) {
                if (!onlyOftStore) {
                    throw new Error(
                        'If you want to proceed with only the OFT Store having the ability to mint, please specify --only-oft-store true. Note that this also means the Freeze Authority will be immediately renounced, unless --freeze-authority is specified.'
                    )
                }
            }

            if (freezeAuthorityStr && !onlyOftStore) {
                throw new Error('`--freeze-authority` is only supported in `--only-oft-store true` mode')
            }

            if (onlyOftStore && additionalMintersAsStrings && additionalMintersAsStrings?.length > 0) {
                throw new Error(
                    'Cannot set both --only-oft-store and --additional-minters; these options are mutually exclusive.'
                )
            }

            if (onlyOftStore && !ci) {
                const continueWithOnlyOftStore = await promptToContinue(
                    `You have chosen \`--only-oft-store true\`. This means that only the OFT Store will be able to mint new tokens${freezeAuthorityStr ? '' : ' and that the Freeze Authority will be immediately renounced'}.  Continue?`
                )
                if (!continueWithOnlyOftStore) {
                    return
                }
            }

            // EOF: Validate combination of parameters

            const maxSupplyRaw = localDecimalsToMaxWholeTokens(decimals)
            const { full, compact } = formatTokenAmount(maxSupplyRaw)
            const maxSupplyStatement = `You have chosen ${decimals} local decimals. The maximum supply of your Solana OFT token will be ${full} (~${compact}).\n`
            const confirmMaxSupply = await promptToContinue(maxSupplyStatement)
            if (!confirmMaxSupply) {
                return
            }

            let mintAuthorityPublicKey: PublicKey = toWeb3JsPublicKey(oftStorePda) // we default to the OFT Store as the Mint Authority when there are no additional minters

            if (additionalMintersAsStrings) {
                // we only need a multisig when we have additional minters
                mintAuthorityPublicKey = await createMintAuthorityMultisig(
                    connection,
                    umi,
                    eid,
                    umiWalletSigner,
                    toWeb3JsPublicKey(oftStorePda),
                    toWeb3JsPublicKey(tokenProgramId), // Only configurable for MABA
                    additionalMinters,
                    computeUnitPriceScaleFactor
                )
                console.log(`created SPL multisig @ ${mintAuthorityPublicKey.toBase58()}`)
                await checkMultisigSigners(connection, mintAuthorityPublicKey, [
                    toWeb3JsPublicKey(oftStorePda),
                    ...additionalMinters,
                ])
            }

            // BOF: determine freeze authority
            // important: this must be placed after multisig creation
            let freezeAuthority: PublicKey | null = null
            if (freezeAuthorityStr && onlyOftStore) {
                freezeAuthority = new PublicKey(freezeAuthorityStr) // will error if invalid
                const continueFreezeAuthority = await promptToContinue(
                    `Freeze Authority will be set to ${freezeAuthority.toBase58()}. Continue?`
                )
                if (!continueFreezeAuthority) {
                    return
                }
            } else {
                // onlyOftStore mode: if freezeAuthority is not provided, we set it to null
                // additional minters mode: set freezeAuthority to mintAuthorityPublicKey (SPL Multisig)
                freezeAuthority = onlyOftStore ? null : mintAuthorityPublicKey
            }
            // EOF: determine freeze authority

            const mint = isMABA
                ? createNoopSigner(publicKey(mintStr))
                : createSignerFromKeypair(umi, eddsa.generateKeypair())
            const isTestnet = eid == EndpointId.SOLANA_V2_TESTNET
            if (!isMABA) {
                const createV1Args: CreateV1InstructionAccounts & CreateV1InstructionArgs = {
                    mint,
                    name,
                    symbol,
                    decimals,
                    uri,
                    isMutable,
                    sellerFeeBasisPoints: percentAmount(sellerFeeBasisPoints),
                    authority: umiWalletSigner, // authority is transferred later
                    tokenStandard: TokenStandard.Fungible,
                }
                let txBuilder = transactionBuilder().add(createV1(umi, createV1Args))
                if (amount) {
                    // recreate txBuilder since it is immutable
                    txBuilder = transactionBuilder()
                        .add(txBuilder)
                        .add(
                            mintV1(umi, {
                                ...createV1Args,
                                mint: publicKey(createV1Args.mint),
                                authority: umiWalletSigner,
                                amount,
                                tokenOwner: umiWalletSigner.publicKey,
                                tokenStandard: TokenStandard.Fungible,
                            })
                        )
                }
                txBuilder = await addComputeUnitInstructions(
                    connection,
                    umi,
                    eid,
                    txBuilder,
                    umiWalletSigner,
                    computeUnitPriceScaleFactor,
                    TransactionType.CreateToken
                )
                const createTokenTx = await txBuilder.sendAndConfirm(umi)
                await assertAccountInitialized(connection, toWeb3JsPublicKey(mint.publicKey))
                console.log(`createTokenTx: ${getExplorerTxLink(bs58.encode(createTokenTx.signature), isTestnet)}`)
            }

            const lockboxSigner = createSignerFromKeypair({ eddsa: eddsa }, lockBox)
            let txBuilder = transactionBuilder().add(
                oft.initOft(
                    {
                        payer: umiWalletSigner,
                        admin: umiWalletKeyPair.publicKey,
                        mint: mint.publicKey,
                        escrow: lockboxSigner,
                    },
                    oft.types.OFTType.Native,
                    sharedDecimals,
                    {
                        oft: programId,
                        token: tokenProgramId,
                    }
                )
            )
            txBuilder = await addComputeUnitInstructions(
                connection,
                umi,
                eid,
                txBuilder,
                umiWalletSigner,
                computeUnitPriceScaleFactor,
                TransactionType.InitOft
            )
            const { signature } = await txBuilder.sendAndConfirm(umi)
            console.log(`initOftTx: ${getExplorerTxLink(bs58.encode(signature), isTestnet)}`)

            if (!isMABA) {
                let txBuilder = transactionBuilder()
                    .add(
                        setAuthority(umi, {
                            owned: mint.publicKey,
                            owner: umiWalletSigner,
                            newAuthority: fromWeb3JsPublicKey(mintAuthorityPublicKey),
                            authorityType: AuthorityType.MintTokens,
                        })
                    )
                    .add(
                        setAuthority(umi, {
                            owned: mint.publicKey,
                            owner: umiWalletSigner,
                            newAuthority: freezeAuthority ? fromWeb3JsPublicKey(freezeAuthority) : null,
                            authorityType: AuthorityType.FreezeAccount,
                        })
                    )
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
                console.log(`setAuthorityTx: ${getExplorerTxLink(bs58.encode(signature), isTestnet)}`)
            }
            if (isMABA) {
                console.log(
                    `Please note that for MABA mode, you must carry out the change of Mint Authority before making any cross-chain transfers. For more details: https://github.com/LayerZero-Labs/devtools/tree/main/examples/oft-solana#for-oft-mint-and-burn-adapter-maba`
                )
            }
            saveSolanaDeployment(
                eid,
                programIdStr,
                mint.publicKey,
                mintAuthorityPublicKey.toBase58(),
                escrowPK,
                oftStorePda
            )
        }
    )
