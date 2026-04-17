import { createFileRoute } from "@tanstack/react-router"

import { UserDecryptMulti } from "../components/user-decrypt-multi"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface UserDecryptMultiSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/user-decrypt-multi")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): UserDecryptMultiSearch => {
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
    component: UserDecryptMultiRoute,
})

function UserDecryptMultiRoute() {
    const { type, config } = Route.useSearch()
    return <UserDecryptMulti decryptType={type} config={config} />
}
