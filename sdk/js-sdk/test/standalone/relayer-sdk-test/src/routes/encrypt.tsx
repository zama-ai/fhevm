import { createFileRoute } from "@tanstack/react-router"

import { Encrypt } from "../components/encrypt"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"

interface EncryptSearch {
    config: FhevmChainName
}

export const Route = createFileRoute("/encrypt")({
    validateSearch: (search: { config: string }): EncryptSearch => ({
        config: normalizeRelayerConfig(search.config),
    }),
    component: EncryptRoute,
})

function EncryptRoute() {
    const { config } = Route.useSearch()
    return <Encrypt config={config} />
}
