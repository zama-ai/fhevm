import type { FhevmChainConfig } from "../utils/const"

interface ConfigProps {
    fhevmChainConfig: FhevmChainConfig
}

export const Config = ({ fhevmChainConfig }: ConfigProps) => {
    return (
        <div>
            <p>Relayer URL: {fhevmChainConfig.chain.fhevm.relayerUrl}</p>
        </div>
    )
}
