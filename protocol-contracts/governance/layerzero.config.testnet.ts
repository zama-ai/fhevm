import { EndpointId } from '@layerzerolabs/lz-definitions'
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools'

import type { OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

const sepoliaContract: OmniPointHardhat = {
    eid: EndpointId.SEPOLIA_V2_TESTNET,
    contractName: 'GovernanceOAppSender',
}

const zamaTestnetContract: OmniPointHardhat = {
    eid: EndpointId.ZAMA_V2_TESTNET,
    contractName: 'GovernanceOAppReceiver',
}

// To connect the above chains to each other, we need the following pathway:
// Sepolia -> ZamaGatewayTestnet

// We don't use enforce executor gas options here, so we should ensure `options` is set to the correct value through profiling the gas usage of calling GovernanceOAppReceiver._lzReceive(...) on the destination chain.
// To learn more, read https://docs.layerzero.network/v2/concepts/applications/oapp-standard#execution-options-and-enforced-settings

// With the config generator, pathways declared are automatically bidirectional
// i.e. if you declare A,B there's no need to declare B,A
const pathways: TwoWayConfig[] = [
    [
        sepoliaContract, // Chain A contract
        zamaTestnetContract, // Chain B contract
        // TODO: Add custom ZAMA DVN in next line?
        [['LayerZero Labs'], []], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
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
                contract: zamaTestnetContract,
                config: { owner: process.env.SAFE_PROXY_ADDRESS, delegate: process.env.SAFE_PROXY_ADDRESS },
            },
            {
                contract: sepoliaContract,
                config: { owner: process.env.DAO_ADDRESS, delegate: process.env.DAO_ADDRESS },
            },
        ],
        connections,
    }
}
