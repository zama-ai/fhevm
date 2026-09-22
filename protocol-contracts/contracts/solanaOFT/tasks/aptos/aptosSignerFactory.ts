import {
    OmniPoint,
    OmniSigner,
    OmniTransaction,
    OmniTransactionReceipt,
    OmniTransactionResponse,
    formatEid,
} from '@layerzerolabs/devtools'
import { ChainType, EndpointId, endpointIdToChainType } from '@layerzerolabs/lz-definitions'

export function createAptosSignerFactory(): (
    eid: EndpointId
) => Promise<OmniSigner<OmniTransactionResponse<OmniTransactionReceipt>>> {
    return async function (eid: EndpointId): Promise<OmniSigner<OmniTransactionResponse<OmniTransactionReceipt>>> {
        if (endpointIdToChainType(eid) !== ChainType.APTOS && endpointIdToChainType(eid) !== ChainType.INITIA) {
            throw new Error(`createAptosSignerFactory() called with Move VM EID: ${formatEid(eid)}`)
        }

        const aptosSigner: OmniSigner<OmniTransactionResponse<OmniTransactionReceipt>> = {
            // The devtools signature requires these members:
            eid,
            getPoint: () => {
                const point: OmniPoint = {
                    eid,
                    address: '0x0',
                }
                return point
            },

            /**
             * sign(omniTx) => Promise<string>
             * Build & sign an Aptos entry function call, returning the BCS as a hex string.
             */
            sign: async (omniTx: OmniTransaction): Promise<string> => {
                return '0x0'
            },

            /**
             * signAndSend(omniTx) => Promise<OmniTransactionResponse<OmniTransactionReceipt>>
             * Just calls sign(...) to get the BCS hex, then submit it.
             */
            signAndSend: async (omniTx: OmniTransaction) => {
                return {
                    transactionHash: '0x0',
                    wait: async (_confirmations?: number) => {
                        return { transactionHash: '0x0' }
                    },
                }
            },
        }

        return aptosSigner
    }
}
