import type { Hex } from "viem"

import type { DecryptionTypes, RelayerConfigName } from "./utils"

interface PublicDecryptHandle {
    clear: string[]
    handle: Hex[]
}

export interface UserDecryptHandle extends PublicDecryptHandle {
    user: string
    contract: string
}

interface PublicDecryptHandles {
    bool: PublicDecryptHandle
    uint8: PublicDecryptHandle
    uint128: PublicDecryptHandle
    address: PublicDecryptHandle
    mixed: PublicDecryptHandle
}

interface UserDecryptHandles {
    bool: UserDecryptHandle
    uint8: UserDecryptHandle
    uint128: UserDecryptHandle
    address: UserDecryptHandle
    mixed: UserDecryptHandle
}

export const devnetPublicHandles: PublicDecryptHandles = {
    bool: {
        clear: ["false"],
        handle: [
            "0xe4afc7df2125874519ced7d81640b507672d40a801000000000000aa36a70000",
        ],
    },
    uint8: {
        clear: ["92"],
        handle: [
            "0x81b4f92eac67594b811842b644406724d9df869152000000000000aa36a70200",
        ],
    },
    uint128: {
        clear: ["176629652319286571959228378485494008132"],
        handle: [
            "0x9e803337fe190e1fbe0bea8f41ac1229f6d23fca7f000000000000aa36a70600",
        ],
    },
    address: {
        clear: ["0x6b69b3364793E8019b886cB2769B19D7BDE5b7B7"],
        handle: [
            "0x56f625121a6e49be41de05b36ce8a1022b71e8dabc000000000000aa36a70700",
        ],
    },
    mixed: {
        clear: [
            "false",
            "92",
            "176629652319286571959228378485494008132",
            "0x6b69b3364793E8019b886cB2769B19D7BDE5b7B7",
        ],
        handle: [
            "0xe4afc7df2125874519ced7d81640b507672d40a801000000000000aa36a70000",
            "0x81b4f92eac67594b811842b644406724d9df869152000000000000aa36a70200",
            "0x9e803337fe190e1fbe0bea8f41ac1229f6d23fca7f000000000000aa36a70600",
            "0x56f625121a6e49be41de05b36ce8a1022b71e8dabc000000000000aa36a70700",
        ],
    },
}

const testnetPublicHandles: PublicDecryptHandles = {
    bool: {
        clear: ["true"],
        handle: [
            "0xeec4116b5284c85ec525e408153526921aef8bcf5f000000000000aa36a70000",
        ],
    },
    uint8: {
        clear: ["238"],
        handle: [
            "0x33eff35258961f472787e4ebb8a027b46236f78d47010000000000aa36a70200",
        ],
    },
    uint128: {
        clear: ["210395372269620041727244987423642516936"],
        handle: [
            "0x0de5dcd427b5e2b9e508db5b2f0fe614a9b6adb4a3020000000000aa36a70600",
        ],
    },
    address: {
        clear: ["0xE41e1F81284b7EE871749F5396a1902fd8E489cB"],
        handle: [
            "0xcc735950ce4b1018dca2258b9f75054125e26e5d72030000000000aa36a70700",
        ],
    },
    mixed: {
        clear: [
            "true",
            "238",
            "210395372269620041727244987423642516936",
            "0xE41e1F81284b7EE871749F5396a1902fd8E489cB",
        ],
        handle: [
            "0xeec4116b5284c85ec525e408153526921aef8bcf5f000000000000aa36a70000",
            "0x33eff35258961f472787e4ebb8a027b46236f78d47010000000000aa36a70200",
            "0x0de5dcd427b5e2b9e508db5b2f0fe614a9b6adb4a3020000000000aa36a70600",
            "0xcc735950ce4b1018dca2258b9f75054125e26e5d72030000000000aa36a70700",
        ],
    },
}

export const devnetUserHandles: UserDecryptHandles = {
    bool: {
        clear: ["true"],
        handle: [
            "0x17e35e5a623207e430bb0cb172983b668218f95e5d000000000000aa36a70000",
        ],
        contract: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint8: {
        clear: ["172"],
        handle: [
            "0xcac8a03f29416c98415b4a07db8a2e81221a31ee09010000000000aa36a70200",
        ],
        contract: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint128: {
        clear: ["96704876906145363692137296689969936284"],
        handle: [
            "0xae5322e0812f7e2ad530125aa6aae1324cbc2372a8020000000000aa36a70600",
        ],
        contract: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    address: {
        clear: ["0x2AfB4223D14d5e78197352199Cc98BCBd9a73cb3"],
        handle: [
            "0x87b376d56b65d4a6f9140ef1189657c24e19d7e5e2030000000000aa36a70700",
        ],
        contract: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    mixed: {
        clear: [
            "true",
            "172",
            "96704876906145363692137296689969936284",
            "0x2AfB4223D14d5e78197352199Cc98BCBd9a73cb3",
        ],
        handle: [
            "0x17e35e5a623207e430bb0cb172983b668218f95e5d000000000000aa36a70000",
            "0xcac8a03f29416c98415b4a07db8a2e81221a31ee09010000000000aa36a70200",
            "0xae5322e0812f7e2ad530125aa6aae1324cbc2372a8020000000000aa36a70600",
            "0x87b376d56b65d4a6f9140ef1189657c24e19d7e5e2030000000000aa36a70700",
        ],
        contract: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
}

export const testnetUserHandles: UserDecryptHandles = {
    bool: {
        clear: ["false"],
        handle: [
            "0x76620922c5252bd1ec23560f9ece7b1b3fdc5ef3c4000000000000aa36a70000",
        ],
        contract: "0x587CefedEA1dD8b937254184B30625a819B447d5",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint8: {
        clear: ["171"],
        handle: [
            "0x83ec62f8f6e80b4f6f6fd6ff4ebf4ea3798c8d45b1010000000000aa36a70200",
        ],
        contract: "0x587CefedEA1dD8b937254184B30625a819B447d5",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint128: {
        clear: ["85473249764448751399817406145246842283"],
        handle: [
            "0xd32d3e4ff8e89c5080b16fe37450460f4accac5c13020000000000aa36a70600",
        ],
        contract: "0x587CefedEA1dD8b937254184B30625a819B447d5",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    address: {
        clear: ["0xFFD36410664583CC0AFa7434bc39A270bc468846"],
        handle: [
            "0x784d60d02656f536a2e97259f8e53ba9113074204b030000000000aa36a70700",
        ],
        contract: "0x587CefedEA1dD8b937254184B30625a819B447d5",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    mixed: {
        clear: [
            "false",
            "171",
            "85473249764448751399817406145246842283",
            "0xFFD36410664583CC0AFa7434bc39A270bc468846",
        ],
        handle: [
            "0x76620922c5252bd1ec23560f9ece7b1b3fdc5ef3c4000000000000aa36a70000",
            "0x83ec62f8f6e80b4f6f6fd6ff4ebf4ea3798c8d45b1010000000000aa36a70200",
            "0xd32d3e4ff8e89c5080b16fe37450460f4accac5c13020000000000aa36a70600",
            "0x784d60d02656f536a2e97259f8e53ba9113074204b030000000000aa36a70700",
        ],
        contract: "0x587CefedEA1dD8b937254184B30625a819B447d5",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
}

export const devnetUserHandlesMirror: UserDecryptHandles = {
    bool: {
        clear: ["false"],
        handle: [
            "0x3d3759fb7ea491b352f6538963980be3ab712b28cf000000000000aa36a70000",
        ],
        contract: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint8: {
        clear: ["252"],
        handle: [
            "0x168067e55ee96ddd4d07025d84d6c8f1753ddca0a4010000000000aa36a70200",
        ],
        contract: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint128: {
        clear: ["99623578138709638110491152906520745990"],
        handle: [
            "0xc15a569977bc38bbc0e376258149b4700c20568895020000000000aa36a70600",
        ],
        contract: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    address: {
        clear: ["0xBCc22721082c1D38d9b475A6440EFe8658F2078F"],
        handle: [
            "0x1855504d16f9670648fbe46074ca70f3ec725a804d030000000000aa36a70700",
        ],
        contract: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    mixed: {
        clear: [
            "false",
            "252",
            "99623578138709638110491152906520745990",
            "0xBCc22721082c1D38d9b475A6440EFe8658F2078F",
        ],
        handle: [
            "0x3d3759fb7ea491b352f6538963980be3ab712b28cf000000000000aa36a70000",
            "0x168067e55ee96ddd4d07025d84d6c8f1753ddca0a4010000000000aa36a70200",
            "0xc15a569977bc38bbc0e376258149b4700c20568895020000000000aa36a70600",
            "0x1855504d16f9670648fbe46074ca70f3ec725a804d030000000000aa36a70700",
        ],
        contract: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
}

export const testnetUserHandlesMirror: UserDecryptHandles = {
    bool: {
        clear: ["false"],
        handle: [
            "0x2c2258c9c25adb06a475ab12c97ec45d45b1a03608000000000000aa36a70000",
        ],
        contract: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint8: {
        clear: ["225"],
        handle: [
            "0x3c6c2ddd6e43af8649da6fd72e33e15a2dbf9856c8010000000000aa36a70200",
        ],
        contract: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    uint128: {
        clear: ["325251480655372520175462643345601539450"],
        handle: [
            "0x84734032d4d5fb6f71e3874091952651b8c9bcbc4b020000000000aa36a70600",
        ],
        contract: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    address: {
        clear: ["0xb8866Bb5c6d2403B78b5b75ebC896760F8AdfB58"],
        handle: [
            "0x4ac9a1103cd8a517dc825b2b62a4c526c7eb5892f0030000000000aa36a70700",
        ],
        contract: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
    mixed: {
        clear: [
            "false",
            "225",
            "325251480655372520175462643345601539450",
            "0xb8866Bb5c6d2403B78b5b75ebC896760F8AdfB58",
        ],
        handle: [
            "0x2c2258c9c25adb06a475ab12c97ec45d45b1a03608000000000000aa36a70000",
            "0x3c6c2ddd6e43af8649da6fd72e33e15a2dbf9856c8010000000000aa36a70200",
            "0x84734032d4d5fb6f71e3874091952651b8c9bcbc4b020000000000aa36a70600",
            "0x4ac9a1103cd8a517dc825b2b62a4c526c7eb5892f0030000000000aa36a70700",
        ],
        contract: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
        user: "0xE2f899d22b00854D8B4d2704D9429be9C411c433",
    },
}

export const getPublicHandles = (
    config: RelayerConfigName,
    decryptType: DecryptionTypes
): PublicDecryptHandle => {
    switch (config) {
        case "devnet":
        case "devnetV2":
            return devnetPublicHandles[decryptType]
        case "testnet":
        case "testnetV2":
            return testnetPublicHandles[decryptType]
    }
}

export const getUserHandles = (
    config: RelayerConfigName,
    decryptType: DecryptionTypes,
    isMirror = false
): UserDecryptHandle => {
    switch (config) {
        case "devnet":
        case "devnetV2":
            return isMirror
                ? devnetUserHandlesMirror[decryptType]
                : devnetUserHandles[decryptType]
        case "testnet":
        case "testnetV2":
            return isMirror
                ? testnetUserHandlesMirror[decryptType]
                : testnetUserHandles[decryptType]
    }
}
