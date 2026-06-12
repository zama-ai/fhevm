import { EndpointId } from '@layerzerolabs/lz-definitions'
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools'
import { OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

const sepoliaContract: OmniPointHardhat = {
    eid: EndpointId.SEPOLIA_V2_TESTNET,
    contractName: 'ConfidentialBridge',
}

const polygonAmoyContract: OmniPointHardhat = {
    eid: EndpointId.AMOY_V2_TESTNET,
    contractName: 'ConfidentialBridge',
}

const pathways: TwoWayConfig[] = [
    [
        sepoliaContract,
        polygonAmoyContract,
        // [requiredDVN[], [optionalDVN[], threshold]] — TESTNET-grade single DVN.
        [['LayerZero Labs'], []], // WARNING: This is a testnet configuration, for production use at least 2 independent operators, e.g. [['LayerZero Labs', 'Google Cloud'], []].
        // Block confirmations [src→dst, dst→src].
        [1, 1], // WARNING: This is a testnet configuration, for production use 15 and 120 confirmations respectively.
        [undefined, undefined],
    ],
]

export default async function () {
    // Generate the connections config based on the pathways
    const connections = await generateConnectionsConfig(pathways)
    return {
        contracts: [{ contract: sepoliaContract }, { contract: polygonAmoyContract }],
        connections,
    }
}
