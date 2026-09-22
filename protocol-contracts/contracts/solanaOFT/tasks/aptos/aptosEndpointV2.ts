import { OmniAddress, OmniPoint, OmniTransaction } from '@layerzerolabs/devtools'
import { EndpointId } from '@layerzerolabs/lz-definitions'
import {
    IEndpointV2,
    MessageParams,
    MessagingFee,
    SetConfigParam,
    Timeout,
    Uln302ConfigType,
    Uln302ExecutorConfig,
    Uln302SetExecutorConfig,
    Uln302SetUlnConfig,
    Uln302UlnConfig,
    UlnReadSetUlnConfig,
    UlnReadUlnConfig,
    UlnReadUlnUserConfig,
} from '@layerzerolabs/protocol-devtools'

/**
 * Minimal "AptosEndpointV2" skeleton that implements IEndpointV2 for Aptos.
 * All methods here return placeholder or dummy values for now.
 */
export class AptosEndpointV2 implements IEndpointV2 {
    // The devtools often expect the "point" property from IOmniSDK
    public point: OmniPoint

    constructor(point: OmniPoint) {
        this.point = point
    }

    //
    // ----------------------------------------------------------
    //  Required by IOmniSDK (the base) or the IEndpointV2 extension
    // ----------------------------------------------------------

    // The devtools might call this to fetch a "Uln302" object.
    // For now, return a dummy object that implements IUln302
    async getUln302SDK(_address: OmniAddress) {
        return {
            point: this.point,
            // placeholders to avoid compile errors:
            async getUlnConfig(_eid: EndpointId, _address: OmniAddress, _type: Uln302ConfigType) {
                return {} as Uln302UlnConfig
            },
            async getAppUlnConfig(_eid: EndpointId, _address: OmniAddress, _type: Uln302ConfigType) {
                return {} as Uln302UlnConfig
            },
            async hasAppUlnConfig(
                _eid: EndpointId,
                _oapp: OmniAddress,
                _config: Uln302UlnConfig,
                _type: Uln302ConfigType
            ) {
                return false
            },
            async setDefaultUlnConfig(_eid: EndpointId, _config: Uln302UlnConfig) {
                return {
                    point: this.point,
                    data: '0x00',
                } as OmniTransaction
            },
            async getExecutorConfig(_eid: EndpointId, _address: OmniAddress) {
                return {
                    maxMessageSize: 1024,
                    executor: '0x0',
                } as Uln302ExecutorConfig
            },
            async getAppExecutorConfig(_eid: EndpointId, _address: OmniAddress): Promise<Uln302ExecutorConfig> {
                return {
                    maxMessageSize: 1024,
                    executor: '0x0',
                } as Uln302ExecutorConfig
            },
            async hasAppExecutorConfig(_eid: EndpointId, _oapp: OmniAddress, _config: Uln302ExecutorConfig) {
                return false
            },
            async setDefaultExecutorConfig(_eid: EndpointId, _config: Uln302ExecutorConfig) {
                return {
                    point: this.point,
                    data: '0x00',
                } as OmniTransaction
            },
        }
    }

    // The devtools might call this to fetch a "UlnRead" object.
    // For now, return a dummy object that implements IUlnRead
    async getUlnReadSDK(_address: OmniAddress) {
        return {
            point: this.point,
            async getUlnConfig(_channelId: number, _address: OmniAddress) {
                return {
                    executor: '0x0',
                    requiredDVNs: [],
                    optionalDVNs: [],
                    optionalDVNThreshold: 0,
                } as UlnReadUlnConfig
            },
            async getAppUlnConfig(_channelId: number, _address: OmniAddress) {
                return {
                    executor: '0x0',
                    requiredDVNs: [],
                    optionalDVNs: [],
                    optionalDVNThreshold: 0,
                } as UlnReadUlnConfig
            },
            async hasAppUlnConfig(_channelId: number, _oapp: OmniAddress, _config: UlnReadUlnConfig) {
                return false
            },
            async setDefaultUlnConfig(_channelId: number, _config: UlnReadUlnConfig) {
                return {
                    point: this.point,
                    data: '0x00',
                } as OmniTransaction
            },
        }
    }

    //
    // ----------------------------------------------------------
    //  Required by IEndpointV2 specifically
    // ----------------------------------------------------------

    async getDelegate(_oapp: OmniAddress): Promise<OmniAddress | undefined> {
        return undefined
    }

    async isDelegate(_oapp: OmniAddress, _delegate: OmniAddress): Promise<boolean> {
        return false
    }

    async getDefaultReceiveLibrary(_eid: EndpointId): Promise<OmniAddress | undefined> {
        return undefined
    }

    async setDefaultReceiveLibrary(_eid: EndpointId, _uln: OmniAddress, _gracePeriod?: bigint) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async getDefaultSendLibrary(_eid: EndpointId): Promise<OmniAddress | undefined> {
        return undefined
    }

    async setDefaultSendLibrary(_eid: EndpointId, _uln: OmniAddress) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async isRegisteredLibrary(_uln: OmniAddress): Promise<boolean> {
        return false
    }

    async registerLibrary(_uln: OmniAddress) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async getSendLibrary(_sender: OmniAddress, _dstEid: EndpointId): Promise<OmniAddress | undefined> {
        return '0x0'
    }

    async getReceiveLibrary(_receiver: OmniAddress, _srcEid: EndpointId): Promise<[OmniAddress | undefined, boolean]> {
        return ['0x0', false]
    }

    async getDefaultReceiveLibraryTimeout(_eid: EndpointId): Promise<Timeout> {
        return { lib: '0x0', expiry: BigInt(0) }
    }

    async getReceiveLibraryTimeout(_receiver: OmniAddress, _srcEid: EndpointId): Promise<Timeout> {
        return { lib: '0x0', expiry: BigInt(0) }
    }

    async setSendLibrary(_oapp: OmniAddress, _eid: EndpointId, _uln: OmniAddress) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async isDefaultSendLibrary(_sender: OmniAddress, _dstEid: EndpointId): Promise<boolean> {
        return false
    }

    async setReceiveLibrary(_oapp: OmniAddress, _eid: EndpointId, _uln: OmniAddress, _gracePeriod: bigint) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async setReceiveLibraryTimeout(_oapp: OmniAddress, _eid: EndpointId, _uln: OmniAddress, _expiry: bigint) {
        return {
            point: this.point,
            data: '0x00',
        } as OmniTransaction
    }

    async getExecutorConfig(_oapp: OmniAddress, _uln: OmniAddress, _eid: EndpointId) {
        return {
            maxMessageSize: 1024,
            executor: '0x0',
        } as Uln302ExecutorConfig
    }

    async getAppExecutorConfig(_oapp: OmniAddress, _uln: OmniAddress, _eid: EndpointId) {
        return {
            maxMessageSize: 1024,
            executor: '0x0',
        } as Uln302ExecutorConfig
    }

    async hasAppExecutorConfig(_oapp: OmniAddress, _uln: OmniAddress, _eid: EndpointId, _config: Uln302ExecutorConfig) {
        return false
    }

    async setExecutorConfig(_oapp: OmniAddress, _uln: OmniAddress, _setExecutorConfig: Uln302SetExecutorConfig[]) {
        // Possibly return multiple OmniTransactions if the devtools call expects them
        return [
            {
                point: this.point,
                data: '0x00',
            },
        ] as OmniTransaction[]
    }

    async getUlnConfig(_oapp: OmniAddress, _uln: OmniAddress, _eid: EndpointId, _type: Uln302ConfigType) {
        return {
            confirmations: BigInt(0),
            requiredDVNs: [],
            optionalDVNs: [],
            optionalDVNThreshold: 0,
        } as Uln302UlnConfig
    }

    async getAppUlnConfig(_oapp: OmniAddress, _uln: OmniAddress, _eid: EndpointId, _type: Uln302ConfigType) {
        return {
            confirmations: BigInt(0),
            requiredDVNs: [],
            optionalDVNs: [],
            optionalDVNThreshold: 0,
        } as Uln302UlnConfig
    }

    async getAppUlnReadConfig(_oapp: OmniAddress, _uln: OmniAddress, _channelId: number) {
        return {
            executor: '0x0',
            requiredDVNs: [],
            optionalDVNs: [],
            optionalDVNThreshold: 0,
        } as UlnReadUlnConfig
    }

    async hasAppUlnConfig(
        _oapp: OmniAddress,
        _uln: OmniAddress,
        _eid: EndpointId,
        _config: any,
        _type: Uln302ConfigType
    ) {
        return false
    }

    async hasAppUlnReadConfig(
        _oapp: OmniAddress,
        _uln: OmniAddress,
        _channelId: number,
        _config: UlnReadUlnUserConfig
    ) {
        return false
    }

    async setUlnConfig(_oapp: OmniAddress, _uln: OmniAddress, _setUlnConfig: Uln302SetUlnConfig[]) {
        return [
            {
                point: this.point,
                data: '0x00',
            },
        ] as OmniTransaction[]
    }

    async setUlnReadConfig(_oapp: OmniAddress, _uln: OmniAddress, _setUlnConfig: UlnReadSetUlnConfig[]) {
        return [
            {
                point: this.point,
                data: '0x00',
            },
        ] as OmniTransaction[]
    }

    async getUlnConfigParams(_uln: OmniAddress, _setUlnConfig: Uln302SetUlnConfig[]) {
        return [] as SetConfigParam[]
    }

    async getUlnReadConfigParams(_uln: OmniAddress, _setUlnConfig: UlnReadSetUlnConfig[]) {
        return [] as SetConfigParam[]
    }

    async getExecutorConfigParams(_uln: OmniAddress, _setExecutorConfig: Uln302SetExecutorConfig[]) {
        return [] as SetConfigParam[]
    }

    async setConfig(_oapp: OmniAddress, _uln: OmniAddress, _setConfigParam: SetConfigParam[]) {
        return [
            {
                point: this.point,
                data: '0x00',
            },
        ] as OmniTransaction[]
    }

    async quote(_params: MessageParams, _sender: OmniAddress): Promise<MessagingFee> {
        return {
            nativeFee: BigInt(0),
            lzTokenFee: BigInt(0),
        }
    }
}
