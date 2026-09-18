import { OmniAddress, OmniPoint, OmniTransaction } from '@layerzerolabs/devtools'
import { ChainType, EndpointId, endpointIdToChainType } from '@layerzerolabs/lz-definitions'
import { IOApp, OAppEnforcedOptionParam } from '@layerzerolabs/ua-devtools'

import { AptosEndpointV2 } from './aptosEndpointV2'

export function createAptosOAppFactory() {
    return async function (point: OmniPoint): Promise<IOApp> {
        const supportedChaintypes = [ChainType.APTOS, ChainType.INITIA]
        if (!supportedChaintypes.includes(endpointIdToChainType(point.eid))) {
            throw new Error(`Aptos SDK factory can only create SDKs for Aptos networks. Received EID ${point.eid}.`)
        }

        const createStubTransaction = (description: string): OmniTransaction => ({
            point,
            data: `0x`,
            description: `[APTOS STUB] ${description}`,
        })

        return {
            point,
            async getOwner(): Promise<OmniAddress | undefined> {
                return undefined
            },
            async hasOwner(owner: OmniAddress): Promise<boolean> {
                return false
            },
            async setOwner(owner: OmniAddress): Promise<OmniTransaction> {
                return createStubTransaction(`setOwner(${owner})`)
            },
            async getEndpointSDK() {
                return new AptosEndpointV2(point)
            },
            async getPeer(eid: EndpointId): Promise<OmniAddress | undefined> {
                return undefined
            },
            async hasPeer(eid: EndpointId, peer: OmniAddress): Promise<boolean> {
                return false
            },
            async setPeer(eid: EndpointId, peer: OmniAddress | null | undefined): Promise<OmniTransaction> {
                return createStubTransaction(`setPeer(${eid}, ${peer})`)
            },
            async getDelegate(): Promise<OmniAddress | undefined> {
                return undefined
            },
            async setDelegate(address: OmniAddress): Promise<OmniTransaction> {
                return createStubTransaction(`setDelegate(${address})`)
            },
            async isDelegate(): Promise<boolean> {
                return false
            },
            async getEnforcedOptions(): Promise<any> {
                return {}
            },
            async setEnforcedOptions(enforcedOptions: OAppEnforcedOptionParam[]): Promise<OmniTransaction> {
                return createStubTransaction(`setEnforcedOptions(${enforcedOptions.length} options)`)
            },
            async getCallerBpsCap(): Promise<bigint | undefined> {
                return BigInt(0)
            },
            async setCallerBpsCap(callerBpsCap: bigint): Promise<OmniTransaction | undefined> {
                return createStubTransaction(`setCallerBpsCap(${callerBpsCap})`)
            },
        }
    }
}
