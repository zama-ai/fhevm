import { createFileRoute } from "@tanstack/react-router"

import { PublicDecrypt } from "../components/public-decrypt"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"
import {
    DEFAULT_DECRYPT_TYPE,
    type DecryptType,
    isDecryptType,
} from "../utils/decrypt"

interface PublicDecryptSearch {
    type: DecryptType
    config: FhevmChainName
}

export const Route = createFileRoute("/public-decrypt")({
    validateSearch: (search: {
        config: string
        type: DecryptType
    }): PublicDecryptSearch => {
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
    return <PublicDecrypt decryptType={type} config={config} />
}
