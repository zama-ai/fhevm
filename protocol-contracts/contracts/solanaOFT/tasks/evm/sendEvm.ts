import bs58 from 'bs58'
import { BigNumber, ContractTransaction } from 'ethers'
import { parseUnits } from 'ethers/lib/utils'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { makeBytes32 } from '@layerzerolabs/devtools'
import { createGetHreByEid } from '@layerzerolabs/devtools-evm-hardhat'
import { createLogger, promptToContinue } from '@layerzerolabs/io-devtools'
import { ChainType, endpointIdToChainType, endpointIdToNetwork } from '@layerzerolabs/lz-definitions'

import layerzeroConfig from '../../layerzero.config'
import { SendResult } from '../common/types'
import { DebugLogger, KnownErrors, MSG_TYPE, isEmptyOptionsEvm } from '../common/utils'
import { getLayerZeroScanLink } from '../solana'
const logger = createLogger()
export interface EvmArgs {
    srcEid: number
    dstEid: number
    amount: string
    to: string
    minAmount?: string
    extraOptions?: string
    composeMsg?: string
    oftAddress?: string
}
export async function sendEvm(
    { srcEid, dstEid, amount, to, minAmount, extraOptions, composeMsg, oftAddress }: EvmArgs,
    hre: HardhatRuntimeEnvironment
): Promise<SendResult> {
    if (endpointIdToChainType(srcEid) !== ChainType.EVM) {
        throw new Error(`non-EVM srcEid (${srcEid}) not supported here`)
    }
    const getHreByEid = createGetHreByEid(hre)
    let srcEidHre: HardhatRuntimeEnvironment
    try {
        srcEidHre = await getHreByEid(srcEid)
    } catch (error) {
        DebugLogger.printErrorAndFixSuggestion(
            KnownErrors.ERROR_GETTING_HRE,
            `For network: ${endpointIdToNetwork(srcEid)}, OFT: ${oftAddress}`
        )
        throw error
    }
    const signer = await srcEidHre.ethers.getNamedSigner('deployer')
    // 1️⃣ resolve the OFT wrapper address
    let wrapperAddress: string
    if (oftAddress) {
        wrapperAddress = oftAddress
    } else {
        const { contracts } = typeof layerzeroConfig === 'function' ? await layerzeroConfig() : layerzeroConfig
        const wrapper = contracts.find((c) => c.contract.eid === srcEid)
        if (!wrapper) throw new Error(`No config for EID ${srcEid}`)
        wrapperAddress = wrapper.contract.contractName
            ? (await srcEidHre.deployments.get(wrapper.contract.contractName)).address
            : wrapper.contract.address!
    }
    // 2️⃣ load OFT ABI
    const oftArtifact = await srcEidHre.artifacts.readArtifact('IOFT')
    const oft = await srcEidHre.ethers.getContractAt(oftArtifact.abi, wrapperAddress, signer)
    // 3️⃣ fetch the underlying ERC-20
    const underlying = await oft.token()
    // 4️⃣ fetch decimals from the underlying token
    const erc20 = await srcEidHre.ethers.getContractAt('ERC20', underlying, signer)
    const decimals: number = await erc20.decimals()
    // 5️⃣ normalize the user-supplied amount
    const amountUnits: BigNumber = parseUnits(amount, decimals)
    // Decide how to encode `to` based on target chain:
    const dstChain = endpointIdToChainType(dstEid)
    let toBytes: string
    if (dstChain === ChainType.SOLANA) {
        // Base58→32-byte buffer
        toBytes = makeBytes32(bs58.decode(to))
    } else {
        // hex string → Uint8Array → zero-pad to 32 bytes
        toBytes = makeBytes32(to)
    }

    // 6️⃣ Check if approval is required (for OFT Adapters) and handle approval
    try {
        const approvalRequired = await oft.approvalRequired()
        if (approvalRequired) {
            logger.info('OFT Adapter detected - checking ERC20 allowance...')

            // Check current allowance
            const currentAllowance = await erc20.allowance(signer.address, wrapperAddress)
            logger.info(`Current allowance: ${currentAllowance.toString()}`)
            logger.info(`Required amount: ${amountUnits.toString()}`)

            if (currentAllowance.lt(amountUnits)) {
                logger.info('Insufficient allowance - approving ERC20 tokens...')
                const approveTx = await erc20.approve(wrapperAddress, amountUnits)
                logger.info(`Approval transaction hash: ${approveTx.hash}`)
                await approveTx.wait()
                logger.info('ERC20 approval confirmed')
            } else {
                logger.info('Sufficient allowance already exists')
            }
        }
    } catch (error) {
        // If approvalRequired() doesn't exist or fails, assume it's a regular OFT (not an adapter)
        logger.info('No approval required (regular OFT detected)')
    }

    // 6️⃣ build sendParam and dispatch
    const sendParam = {
        dstEid,
        to: toBytes,
        amountLD: amountUnits.toString(),
        minAmountLD: minAmount ? parseUnits(minAmount, decimals).toString() : amountUnits.toString(),
        extraOptions: extraOptions ? extraOptions.toString() : '0x',
        composeMsg: composeMsg ? composeMsg.toString() : '0x',
        oftCmd: '0x',
    }

    // Check whether there are extra options or enforced options. If not, warn the user.
    // Read on Message Options: https://docs.layerzero.network/v2/concepts/message-options
    if (!extraOptions) {
        try {
            const enforcedOptions = composeMsg
                ? await oft.enforcedOptions(dstEid, MSG_TYPE.SEND_AND_CALL)
                : await oft.enforcedOptions(dstEid, MSG_TYPE.SEND)

            if (isEmptyOptionsEvm(enforcedOptions)) {
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

    // 6️⃣ Quote (MessagingFee = { nativeFee, lzTokenFee })
    logger.info('Quoting the native gas cost for the send transaction...')
    let msgFee: { nativeFee: BigNumber; lzTokenFee: BigNumber }
    try {
        msgFee = await oft.quoteSend(sendParam, false)
    } catch (error) {
        DebugLogger.printErrorAndFixSuggestion(
            KnownErrors.ERROR_QUOTING_NATIVE_GAS_COST,
            `For network: ${endpointIdToNetwork(srcEid)}, OFT: ${oftAddress}`
        )
        throw error
    }
    logger.info('Sending the transaction...')
    let tx: ContractTransaction
    try {
        tx = await oft.send(sendParam, msgFee, signer.address, {
            value: msgFee.nativeFee,
        })
    } catch (error) {
        DebugLogger.printErrorAndFixSuggestion(
            KnownErrors.ERROR_SENDING_TRANSACTION,
            `For network: ${endpointIdToNetwork(srcEid)}, OFT: ${oftAddress}`
        )
        throw error
    }
    const receipt = await tx.wait()
    const txHash = receipt.transactionHash
    const scanLink = getLayerZeroScanLink(txHash, srcEid >= 40_000 && srcEid < 50_000)
    return { txHash, scanLink }
}
