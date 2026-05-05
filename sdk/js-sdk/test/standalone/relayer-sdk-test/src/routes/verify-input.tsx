import { createFileRoute } from "@tanstack/react-router"

import { VerifyInput } from "../components/verify-input"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"

interface VerifyInputSearch {
    config: FhevmChainName
}

export const Route = createFileRoute("/verify-input")({
    validateSearch: (search: { config: string }): VerifyInputSearch => ({
        config: normalizeRelayerConfig(search.config),
    }),
    component: VerifyInputRoute,
})

function VerifyInputRoute() {
    const { config } = Route.useSearch()
    return <VerifyInput config={config} />
}
