// NOTE: This config is an example for wiring Solana -> Aptos.
// See wiring-to-aptos.md for more information.
import { EndpointId } from '@layerzerolabs/lz-definitions'
import { ExecutorOptionType } from '@layerzerolabs/lz-v2-utilities'
import { OAppOmniGraphHardhat, OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat'

enum MsgType {
    SEND = 1,
    SEND_AND_CALL = 2,
}
const aptosContract: OmniPointHardhat = {
    eid: EndpointId.APTOS_V2_TESTNET as EndpointId,
    address: 'YOUR_DEPLOYED_APTOS_OFT_ADDRESS',
}
const solanaContract: OmniPointHardhat = {
    eid: EndpointId.SOLANA_V2_TESTNET as EndpointId,
    address: 'YOUR_DEPLOYED_SOLANA_OAPP_ADDRESS',
}

const layerZeroDVNAptosAddress = '0x756f8ab056688d22687740f4a9aeec3b361170b28d08b719e28c4d38eed1043e'
const layerZeroDVNSolanaAddress = '4VDjp6XQaxoZf5RGwiPU9NR1EXSZn2TP4ATMmiSzLfhb'

const config: OAppOmniGraphHardhat = {
    contracts: [
        {
            contract: solanaContract,
            config: {
                owner: 'YOUR_SOLANA_OWNER_ADDRESS',
                delegate: 'YOUR_SOLANA_OWNER_ADDRESS',
            },
        },
        {
            contract: aptosContract,
            config: {
                owner: 'YOUR_APTOS_ACCOUNT_ADDRESS',
                delegate: 'YOUR_APTOS_ACCOUNT_ADDRESS',
            },
        },
    ],
    connections: [
        {
            from: aptosContract,
            to: solanaContract,
            config: {
                enforcedOptions: [
                    {
                        msgType: MsgType.SEND_AND_CALL,
                        optionType: ExecutorOptionType.LZ_RECEIVE,
                        gas: 5_000,
                        value: 0,
                    },
                ],
                sendLibrary: '0xcc1c03aed42e2841211865758b5efe93c0dde2cb7a2a5dc6cf25a4e33ad23690',
                receiveLibraryConfig: {
                    receiveLibrary: '0xcc1c03aed42e2841211865758b5efe93c0dde2cb7a2a5dc6cf25a4e33ad23690',
                    gracePeriod: BigInt(0),
                },
                sendConfig: {
                    executorConfig: {
                        maxMessageSize: 10_000,
                        executor: '0x93353700091200ef9fdc536ce6a86182cc7e62da25f94356be9421c6310b9585',
                    },
                    ulnConfig: {
                        confirmations: BigInt(10),
                        requiredDVNs: [layerZeroDVNAptosAddress],
                        optionalDVNs: [],
                        optionalDVNThreshold: 0,
                    },
                },
                receiveConfig: {
                    ulnConfig: {
                        confirmations: BigInt(15),
                        requiredDVNs: [layerZeroDVNAptosAddress],
                        optionalDVNs: [],
                        optionalDVNThreshold: 0,
                    },
                },
            },
        },
        {
            from: solanaContract,
            to: aptosContract,
            config: {
                enforcedOptions: [
                    {
                        msgType: 1,
                        optionType: ExecutorOptionType.LZ_RECEIVE,
                        gas: 200_000,
                        value: 0,
                    },
                ],
                sendLibrary: '7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH',
                receiveLibraryConfig: {
                    receiveLibrary: '7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH',
                    gracePeriod: BigInt(0),
                },
                sendConfig: {
                    executorConfig: {
                        maxMessageSize: 10_000,
                        executor: 'AwrbHeCyniXaQhiJZkLhgWdUCteeWSGaSN1sTfLiY7xK',
                    },
                    ulnConfig: {
                        confirmations: BigInt(15),
                        requiredDVNs: [layerZeroDVNSolanaAddress],
                        optionalDVNs: [],
                        optionalDVNThreshold: 0,
                    },
                },
                receiveConfig: {
                    ulnConfig: {
                        confirmations: BigInt(10),
                        requiredDVNs: [layerZeroDVNSolanaAddress],
                        optionalDVNs: [],
                        optionalDVNThreshold: 0,
                    },
                },
            },
        },
    ],
}
export default config
