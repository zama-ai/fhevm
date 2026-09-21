import { fetchMint, fetchToken, findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox'
import { publicKey, transactionBuilder } from '@metaplex-foundation/umi'
import { fromWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import bs58 from 'bs58'

import { createLogger, promptToContinue } from '@layerzerolabs/io-devtools'
import { EndpointId, endpointIdToNetwork } from '@layerzerolabs/lz-definitions'
import { addressToBytes32 } from '@layerzerolabs/lz-v2-utilities'
import { oft } from '@layerzerolabs/oft-v2-solana-sdk'

import { SendResult } from '../common/types'
import { DebugLogger, KnownErrors, isEmptyOptionsSolana } from '../common/utils'

import { parseDecimalToUnits, silenceSolana429 } from './utils'

import {
    TransactionType,
    addComputeUnitInstructions,
    deriveConnection,
    getDefaultAddressLookupTable,
    getLayerZeroScanLink,
    getSolanaDeployment,
} from './index'

const logger = createLogger()

export interface SolanaArgs {
    amount: string
    to: string
    srcEid: EndpointId
    dstEid: EndpointId
    minAmount?: string
    extraOptions?: string
    composeMsg?: string
    oftAddress?: string
    oftProgramId?: string
    tokenProgram?: string
    computeUnitPriceScaleFactor?: number
    addressLookupTables?: string[]
}

export async function sendSolana({
    amount,
    to,
    srcEid,
    dstEid,
    oftAddress,
    oftProgramId,
    tokenProgram: tokenProgramStr,
    computeUnitPriceScaleFactor = 4,
    minAmount,
    extraOptions,
    composeMsg,
    addressLookupTables,
}: SolanaArgs): Promise<SendResult> {
    // 1️⃣ RPC + UMI
    const { connection, umi, umiWalletSigner } = await deriveConnection(srcEid)
    silenceSolana429(connection)
    // 2️⃣ Pick your OFT program ID (override or from deployment)
    const programId = oftProgramId
        ? publicKey(oftProgramId)
        : publicKey(
              (() => {
                  try {
                      return getSolanaDeployment(srcEid).programId
                  } catch (error) {
                      logger.error(`No Program ID found for ${srcEid}: ${error}`)
                      throw error
                  }
              })()
          )

    // 3️⃣ Decide your store PDA (override or from your on‐disk deployment)
    const storePda = oftAddress ? publicKey(oftAddress) : publicKey(getSolanaDeployment(srcEid).oftStore)
    const oftStoreInfo = await oft.accounts.fetchOFTStore(umi, storePda)
    const mintPk = new PublicKey(oftStoreInfo.tokenMint)
    const escrowPk = new PublicKey(oftStoreInfo.tokenEscrow)

    // 4️⃣ Attach token account & check balance
    const tokenProgramId = tokenProgramStr ? publicKey(tokenProgramStr) : fromWeb3JsPublicKey(TOKEN_PROGRAM_ID)
    const tokenAccount = findAssociatedTokenPda(umi, {
        mint: fromWeb3JsPublicKey(mintPk),
        owner: umiWalletSigner.publicKey,
        tokenProgramId,
    })
    if (!tokenAccount) throw new Error(`No token account for mint ${mintPk}`)
    const balance = (await fetchToken(umi, tokenAccount)).amount

    // 5️⃣ Normalize human→base units
    const decimals = (await fetchMint(umi, fromWeb3JsPublicKey(mintPk))).decimals
    const amountUnits = parseDecimalToUnits(amount, decimals)
    if (amountUnits === 0n || amountUnits > balance) {
        throw new Error(`Insufficient balance (need ${amountUnits}, have ${balance})`)
    }

    // Check whether there are extra options or enforced options. If not, warn the user.
    // Read on Message Options: https://docs.layerzero.network/v2/concepts/message-options
    if (!extraOptions) {
        try {
            const enforcedOptionsMap = await oft.getEnforcedOptions(umi.rpc, storePda, dstEid, programId)
            const enforcedOptionsBuffer = composeMsg ? enforcedOptionsMap.sendAndCall : enforcedOptionsMap.send

            if (isEmptyOptionsSolana(enforcedOptionsBuffer)) {
                const proceed = await promptToContinue(
                    'No extra options were included and OFT has no set enforced options. Your quote / send will most likely fail. Continue?'
                )
                if (!proceed) {
                    throw new Error('Aborted due to missing options')
                }
            }
        } catch (error) {
            logger.debug(`Failed to check enforced options: ${error}`)
        }
    }

    // 6️⃣ Lookup table addresses
    const lookupTableAddresses =
        addressLookupTables && addressLookupTables.length > 0
            ? addressLookupTables.map((addr) => publicKey(addr))
            : [(await getDefaultAddressLookupTable(connection, umi, srcEid)).lookupTableAddress]

    // 7️⃣ Quote (use our overridden `programId`)
    logger.info('Quoting the native gas cost for the send transaction...')
    const sendParam = {
        dstEid,
        to: Buffer.from(addressToBytes32(to)),
        amountLd: amountUnits,
        minAmountLd: minAmount ? parseDecimalToUnits(minAmount, decimals) : amountUnits,
        options: extraOptions ? Buffer.from(extraOptions.replace(/^0x/, ''), 'hex') : undefined,
        composeMsg: composeMsg ? Buffer.from(composeMsg.replace(/^0x/, ''), 'hex') : undefined,
    }
    const { nativeFee } = await oft.quote(
        umi.rpc,
        {
            payer: umiWalletSigner.publicKey,
            tokenMint: fromWeb3JsPublicKey(mintPk),
            tokenEscrow: fromWeb3JsPublicKey(escrowPk),
        },
        {
            payInLzToken: false,
            ...sendParam,
        },
        { oft: programId }, // ← use override
        [],
        lookupTableAddresses
    )

    // 8️⃣ Send (again passing `programId`)
    logger.info('Sending the transaction...')
    const ix = await oft.send(
        umi.rpc,
        {
            payer: umiWalletSigner,
            tokenMint: fromWeb3JsPublicKey(mintPk),
            tokenEscrow: fromWeb3JsPublicKey(escrowPk),
            tokenSource: tokenAccount[0],
        },
        {
            nativeFee,
            ...sendParam,
        },
        { oft: programId, token: tokenProgramId } // ← use override
    )

    // 9️⃣ Compute units & submit
    let txB = transactionBuilder().add([ix])
    txB = await addComputeUnitInstructions(
        connection,
        umi,
        srcEid,
        txB,
        umiWalletSigner,
        computeUnitPriceScaleFactor,
        TransactionType.SendOFT,
        lookupTableAddresses
    )
    let txHash: string
    try {
        const { signature } = await txB.sendAndConfirm(umi)
        txHash = bs58.encode(signature)
    } catch (error) {
        DebugLogger.printErrorAndFixSuggestion(
            KnownErrors.ERROR_QUOTING_NATIVE_GAS_COST,
            `For network: ${endpointIdToNetwork(srcEid)}, OFT: ${oftAddress}`
        )
        throw error
    }

    const isTestnet = srcEid === EndpointId.SOLANA_V2_TESTNET

    const scanLink = getLayerZeroScanLink(txHash, isTestnet)

    return { txHash, scanLink }
}
