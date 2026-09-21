import { EndpointId } from '@layerzerolabs/lz-definitions'
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools'

import type { OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

const ethereumContract: OmniPointHardhat = {
    eid: EndpointId.ETHEREUM_V2_MAINNET,
    contractName: 'GovernanceOAppSender',
}

const zamaMainnetContract: OmniPointHardhat = {
    eid: EndpointId.ZAMA_V2_MAINNET,
    contractName: 'GovernanceOAppReceiver',
}

// To connect the above chains to each other, we need the following pathway:
// Ethereum -> ZamaGatewayMainnet

// We don't use enforced executor gas options here, so we should ensure `options` is set to the correct value through profiling the gas usage of calling GovernanceOAppReceiver._lzReceive(...) on the destination chain.
// To learn more, read https://docs.layerzero.network/v2/concepts/applications/oapp-standard#execution-options-and-enforced-settings

const pathways: TwoWayConfig[] = [
    [
        ethereumContract, // Chain A contract
        zamaMainnetContract, // Chain B contract
        // TODO: Add custom ZAMA DVN in next line?
        [['LayerZero Labs'], [['Nethermind', 'Luganodes', 'P2P'], 2]], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
        [15, undefined], // [A to B confirmations, B to A confirmations] // NOTE: `undefined` is used here because we want an uniderectional pathway
        [undefined, undefined], // NOTE: first `undefined` is because we do not enforce gas, since proposals are arbitrary calls. Instead, we will use a gas profiler to calculate the gas needed for proposal execution and adds a relative buffer on top. Second `undefined` is used here because we want an uniderectional pathway.
    ],
]

export default async function () {
    // Generate the connections config based on the pathways
    const connections = await generateConnectionsConfig(pathways)
    return {
        contracts: [
            {
                contract: zamaMainnetContract,
                config: { owner: process.env.SAFE_PROXY_ADDRESS, delegate: process.env.SAFE_PROXY_ADDRESS },
            },
            {
                contract: ethereumContract,
                config: { owner: process.env.DAO_ADDRESS, delegate: process.env.DAO_ADDRESS },
            },
        ],
        connections,
    }
}
