import { EndpointId } from '@layerzerolabs/lz-definitions'
import { ExecutorOptionType } from '@layerzerolabs/lz-v2-utilities'
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools'
import { OAppEnforcedOption } from '@layerzerolabs/toolbox-hardhat'

import type { OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

const sepoliaContract: OmniPointHardhat = {
    eid: EndpointId.SEPOLIA_V2_TESTNET,
    contractName: 'GovernanceOAppSender',
}

const zamaTestnetContract: OmniPointHardhat = {
    eid: EndpointId.ZAMA_V2_TESTNET,
    contractName: 'GovernanceOAppReceiver',
}

// To connect all the above chains to each other, we need the following pathways:
// ZamaGatewayTestnet <-> Sepolia

// For this example's simplicity, we will use the same enforced options values for sending to all chains
// For production, you should ensure `gas` is set to the correct value through profiling the gas usage of calling GovernanceOAppReceiver._lzReceive(...) on the destination chain
// To learn more, read https://docs.layerzero.network/v2/concepts/applications/oapp-standard#execution-options-and-enforced-settings
const EVM_ENFORCED_OPTIONS: OAppEnforcedOption[] = [
    {
        msgType: 1,
        optionType: ExecutorOptionType.LZ_RECEIVE,
        gas: 80000,
        value: 0,
    },
]

// With the config generator, pathways declared are automatically bidirectional
// i.e. if you declare A,B there's no need to declare B,A
const pathways: TwoWayConfig[] = [
    [
        sepoliaContract, // Chain A contract
        zamaTestnetContract, // Chain B contract
        // TODO: Add custom ZAMA DVN in next line?
        [['LayerZero Labs'], []], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
        [15, undefined], // [A to B confirmations, B to A confirmations] // NOTE: undefined is used here because we want an uniderectional pathway
        [EVM_ENFORCED_OPTIONS, undefined], // Chain B enforcedOptions, Chain A enforcedOptions  // NOTE: undefined is used here because we want an uniderectional pathway
    ],
]

export default async function () {
    // Generate the connections config based on the pathways
    const connections = await generateConnectionsConfig(pathways)
    return {
        contracts: [{ contract: zamaTestnetContract }, { contract: sepoliaContract }],
        connections,
    }
}
