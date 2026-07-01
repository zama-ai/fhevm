import { EndpointId } from '@layerzerolabs/lz-definitions'
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools'
import { OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

const ethereumContract: OmniPointHardhat = {
    eid: EndpointId.ETHEREUM_V2_MAINNET,
    contractName: 'ConfidentialBridge',
}

const polygonContract: OmniPointHardhat = {
    eid: EndpointId.POLYGON_V2_MAINNET,
    contractName: 'ConfidentialBridge',
}

const pathways: TwoWayConfig[] = [
    [
        ethereumContract,
        polygonContract,
        [['LayerZero Labs'], [['Nethermind', 'Luganodes', 'P2P'], 2]], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
        [15, 120], // [A to B confirmations, B to A confirmations]
        [undefined, undefined],
    ],
]

export default async function () {
    // Generate the connections config based on the pathways
    const connections = await generateConnectionsConfig(pathways)
    return {
        contracts: [{ contract: ethereumContract }, { contract: polygonContract }],
        connections,
    }
}
