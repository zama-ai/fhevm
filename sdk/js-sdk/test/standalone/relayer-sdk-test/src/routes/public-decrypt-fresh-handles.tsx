import { createFileRoute } from "@tanstack/react-router"

import { PublicDecryptFreshHandles } from "../components/public-decrypt-fresh-handles"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface PublicDecryptFreshHandlesSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/public-decrypt-fresh-handles")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): PublicDecryptFreshHandlesSearch => {
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
    component: PublicDecryptRoute,
})

function PublicDecryptRoute() {
    const { type, config } = Route.useSearch()
    return <PublicDecryptFreshHandles decryptType={type} config={config} />
}
