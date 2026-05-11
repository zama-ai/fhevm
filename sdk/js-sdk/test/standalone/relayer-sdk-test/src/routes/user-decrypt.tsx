import { createFileRoute } from "@tanstack/react-router"

import { UserDecrypt } from "../components/user-decrypt"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface UserDecryptSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/user-decrypt")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): UserDecryptSearch => {
        const type = search.type
        if (isDecryptType(type)) {
            return {
                type,
                config: normalizeRelayerConfig(search.config),
            }
        }
        return {
            type: DEFAULT_DECRYPT_TYPE,
            config: normalizeRelayerConfig(search.config),
        }
    },
    component: UserDecryptRoute,
})

function UserDecryptRoute() {
    const { type, config } = Route.useSearch()
    return <UserDecrypt decryptType={type} config={config} />
}
