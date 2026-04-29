import { createFileRoute } from "@tanstack/react-router"

import { ZKProof } from "../components/zkproof"
import { type FhevmChainName, normalizeRelayerConfig } from "../utils/const"

interface ZKProofSearch {
    config: FhevmChainName
}

export const Route = createFileRoute("/zkproof")({
    validateSearch: (search: { config: string }): ZKProofSearch => ({
        config: normalizeRelayerConfig(search.config),
    }),
    component: ZKProofRoute,
})

function ZKProofRoute() {
    const { config } = Route.useSearch()
    return <ZKProof config={config} />
}
