import { createFileRoute } from "@tanstack/react-router"

import { Init } from "../components/init"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"

interface InitSearch {
    config: FhevmChainName
}

export const Route = createFileRoute("/init")({
    validateSearch: (search: { config: string }): InitSearch => ({
        config: normalizeRelayerConfig(search.config),
    }),
    component: InitRoute,
})

function InitRoute() {
    const { config } = Route.useSearch()
    return <Init config={config} />
}
