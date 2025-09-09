export interface SendResult {
    txHash: string // EVM: receipt.transactionHash
    scanLink: string // LayerZeroScan link for cross-chain tracking
}
