import { OmniAddress } from '@layerzerolabs/devtools'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { UlnProgram } from '@layerzerolabs/lz-solana-sdk-v2'
import {
    IEndpointV2,
    Timeout,
    Uln302ConfigType,
    Uln302ExecutorConfig,
    Uln302UlnConfig,
} from '@layerzerolabs/protocol-devtools'

/**
 * Get the receive config for a Solana OApp
 * @param endpointV2Sdk {IEndpointV2} SDK for the endpoint
 * @param remoteEid {EndpointId} remote eid
 * @param address {OmniAddress} address of the OApp
 */
export async function getSolanaReceiveConfig(
    endpointV2Sdk: IEndpointV2,
    remoteEid: EndpointId,
    address: OmniAddress
): Promise<[OmniAddress, Uln302UlnConfig, Timeout] | undefined> {
    let receiveLibraryAddress = UlnProgram.PROGRAM_ADDRESS
    try {
        const [receiveLibrary] = await endpointV2Sdk.getReceiveLibrary(address, remoteEid)
        if (receiveLibrary) {
            receiveLibraryAddress = receiveLibrary
        }
    } catch {
        // no action is necessary as upon error, we will just use the default UlnProgram.PROGRAM_ADDRESS
    }
    return [
        receiveLibraryAddress,
        await endpointV2Sdk.getAppUlnConfig(
            address,
            UlnProgram.PROGRAM_ID.toBase58(),
            remoteEid,
            Uln302ConfigType.Receive
        ),
        {
            lib: UlnProgram.PROGRAM_ID.toBase58(),
            expiry: 0n, // unsupported for Solana
        },
    ]
}

/**
 * Get the send config for a Solana OApp.
 * @param endpointV2Sdk {IEndpointV2} SDK for the endpoint
 * @param eid {EndpointId} remote eid
 * @param address {OmniAddress} address of the OApp
 */
export async function getSolanaSendConfig(
    endpointV2Sdk: IEndpointV2,
    eid: EndpointId,
    address: OmniAddress
): Promise<[OmniAddress, Uln302UlnConfig, Uln302ExecutorConfig] | undefined> {
    let sendLibraryAddress = UlnProgram.PROGRAM_ADDRESS
    try {
        const sendLibrary = await endpointV2Sdk.getSendLibrary(address, eid)
        if (sendLibrary) {
            sendLibraryAddress = sendLibrary
        }
    } catch {
        // no action is necessary as upon error, we will just use the default UlnProgram.PROGRAM_ADDRESS
    }
    return [
        sendLibraryAddress,
        await endpointV2Sdk.getAppUlnConfig(address, UlnProgram.PROGRAM_ID.toBase58(), eid, Uln302ConfigType.Send),
        await endpointV2Sdk.getAppExecutorConfig(address, sendLibraryAddress, eid),
    ]
}
