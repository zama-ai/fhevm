import { createFileRoute } from "@tanstack/react-router"

import { UserDecryptFreshHandles } from "../components/user-decrypt-fresh-handles"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface UserDecryptFreshHandlesSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/user-decrypt-fresh-handles")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): UserDecryptFreshHandlesSearch => {
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
    component: UserDecryptFreshHandlesRoute,
})

function UserDecryptFreshHandlesRoute() {
    const { type, config } = Route.useSearch()
    return <UserDecryptFreshHandles decryptType={type} config={config} />
}
