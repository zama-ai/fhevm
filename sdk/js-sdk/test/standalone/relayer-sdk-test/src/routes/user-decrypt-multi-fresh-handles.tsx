import { createFileRoute } from "@tanstack/react-router"

import { UserDecryptMultiFreshHandles } from "../components/user-decrypt-multi-fresh-handles"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface UserDecryptMultiFreshHandlesSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/user-decrypt-multi-fresh-handles")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): UserDecryptMultiFreshHandlesSearch => {
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
    component: UserDecryptMultiFreshHandlesRoute,
})

function UserDecryptMultiFreshHandlesRoute() {
    const { type, config } = Route.useSearch()
    return <UserDecryptMultiFreshHandles decryptType={type} config={config} />
}
