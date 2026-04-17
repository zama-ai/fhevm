import type { RelayerPublicDecryptOptions, RelayerPublicDecryptProgressArgs } from "@fhevm/sdk/actions/base"
import { useCallback, useEffect, useMemo, useState } from "react"
import type { Hex } from "viem"

import { createBaseClient, initBaseRuntime } from "../utils/client"
import { type FhevmChainName, resolveFhevmChainConfig } from "../utils/const"
import { type DecryptType } from "../utils/decrypt"
import { formatDuration, formatError } from "../utils/fmt"
import { getPublicHandles } from "../utils/handles"
import { LogPanel, useLog } from "../utils/log"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import type { BaseClient, PublicDecryptSummary } from "../utils/types"
import { Config } from "./config"
import "./route.css"

interface PublicDecryptProps {
    decryptType: DecryptType
    config: FhevmChainName
}

////////////////////////////////////////////////////////////////////////////////

/**
 * React Component to test public decryption of user-encrypted values with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const PublicDecrypt = ({ decryptType, config }: PublicDecryptProps) => {
    ////////////////////////////////////////////////////////////////////////////
    // States
    ////////////////////////////////////////////////////////////////////////////

    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [decryptState, setDecryptState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [publicDecryptSummary, setPublicDecryptSummary] = useState<PublicDecryptSummary | null>(null)
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
        publicDecrypt?: number
    }>({})

    ////////////////////////////////////////////////////////////////////////////
    // Misc
    ////////////////////////////////////////////////////////////////////////////

    const { lines: logLines, log, clear: clearLog } = useLog()

    const isRunning = initState === "pending" || instanceState === "pending" || decryptState === "pending"
    const relayerConfig = useMemo(() => resolveFhevmChainConfig(config), [config])

    const relayerOptions: RelayerPublicDecryptOptions | undefined = useMemo(() => {
        const option = {
            onProgress(args: RelayerPublicDecryptProgressArgs) {
                log(`V2 ROUTE PROGRESS: ${args.type}`)
            },
        }
        return option
    }, [config])

    ////////////////////////////////////////////////////////////////////////////
    // Reset flow
    ////////////////////////////////////////////////////////////////////////////

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setPublicDecryptSummary(null)
        setDurations({}) // Reset durations on retry

        setInitState("pending")
        setInstanceState("idle")
        setDecryptState("idle")
    }, [])

    ////////////////////////////////////////////////////////////////////////////
    // Step 1: initRuntime
    ////////////////////////////////////////////////////////////////////////////

    const initRuntime = useCallback(async () => {
        setInitState("pending")
        await nextTick()

        const start = performance.now()
        await initBaseRuntime(relayerConfig, log)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, init: end - start }))
        setInitState("success")
    }, [relayerConfig])

    ////////////////////////////////////////////////////////////////////////////
    // Step 2: createClient
    ////////////////////////////////////////////////////////////////////////////

    const createClient = useCallback(async () => {
        setInstanceState("pending")
        await nextTick()

        const start = performance.now()
        const fhevmClient = await createBaseClient(relayerConfig)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, instance: end - start }))
        setInstanceState("success")
        return fhevmClient
    }, [relayerConfig])

    ////////////////////////////////////////////////////////////////////////////
    // Step 3: publicDecryptValue
    ////////////////////////////////////////////////////////////////////////////

    const publicDecryptValue = useCallback(
        async (client: BaseClient, encryptedValues: Hex[]) => {
            setDecryptState("pending")
            await nextTick()

            const start = performance.now()
            const decrypted = await client.readPublicValuesWithSignatures({
                encryptedValues,
                options: relayerOptions,
            })
            const end = performance.now()

            setPublicDecryptSummary({
                clearValues: Object.fromEntries(
                    Object.entries(decrypted.clearValues).map(([key, value]) => [key, value.value.toString()])
                ),
                abiEncodedClearValues: decrypted.checkSignaturesArgs.abiEncodedCleartexts,
                decryptionProof: decrypted.checkSignaturesArgs.decryptionProof,
            })
            setDurations((prev) => ({ ...prev, publicDecrypt: end - start }))
            setDecryptState("success")
        },
        [relayerOptions]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Run Flow
    ////////////////////////////////////////////////////////////////////////////

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initRuntime()
            const fhevmClient = await createClient()
            const handles = getPublicHandles(config, decryptType)
            await publicDecryptValue(fhevmClient, handles.handle)
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setDecryptState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`${decryptType} public decryption flow failed: ${formatError(error)}`)
        }
    }, [resetFlowState, initRuntime, createClient, publicDecryptValue, decryptType])

    ////////////////////////////////////////////////////////////////////////////

    useEffect(() => {
        const timer = window.setTimeout(() => {
            void runFlow()
        }, 0)

        return () => {
            window.clearTimeout(timer)
        }
    }, [runFlow])

    const handleRetry = useCallback(() => {
        void runFlow()
    }, [runFlow])

    const buttonLabel = useMemo(() => {
        if (isRunning) return "Running..."
        return attempts > 0 ? "Rerun flow" : "Run flow"
    }, [attempts, isRunning])

    const formattedCompletedAt = useMemo(() => {
        if (!completedAt) return null
        return new Date(completedAt).toLocaleTimeString()
    }, [completedAt])

    ////////////////////////////////////////////////////////////////////////////

    return (
        <section className="panel">
            {errorMessage ? (
                <div className="status-error">
                    <strong>Public decrypt flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <div>
                    <h2>Public decryption</h2>
                    <div className="chip-row">
                        <span className="chip" data-testid="public-decrypt-selected-type">
                            type={decryptType}
                        </span>
                        <span className="chip muted">Supported: bool, uint8, uint128, address</span>
                    </div>
                </div>
                <Config fhevmChainConfig={relayerConfig}></Config>
                <button className="primary-btn" type="button" onClick={handleRetry} disabled={isRunning}>
                    {buttonLabel}
                </button>
            </header>

            <dl className="status-grid">
                <div className="status-card" data-state={initState}>
                    <dt>
                        initRuntime()
                        {durations.init !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.init)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(initState) ? (
                        <dd data-testid="public-decrypt-init-sdk-status-dd">{statusCopy[initState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={instanceState}>
                    <dt>
                        createClient()
                        {durations.instance !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.instance)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(instanceState) ? (
                        <dd data-testid="public-decrypt-create-instance-status-dd">{statusCopy[instanceState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={decryptState}>
                    <dt>
                        publicDecrypt()
                        {durations.publicDecrypt !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.publicDecrypt)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(decryptState) ? (
                        <dd data-testid="public-decrypt-run-decryption-status-dd">{statusCopy[decryptState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {formattedCompletedAt ? <p className="muted timestamp">Flow completed at {formattedCompletedAt}</p> : null}

            {publicDecryptSummary ? (
                <div className="result-grid">
                    <article className="result-card">
                        <div className="result-card__header">
                            <h3>Public decrypted values</h3>
                            <span className="badge">decrypted</span>
                        </div>
                        <p className="muted small">Clear values</p>
                        <ul className="code-stack">
                            {Object.entries(publicDecryptSummary.clearValues).map(([key, value], index) => (
                                <li key={key}>
                                    <code className="code-block">
                                        <span data-testid={`public-decrypt-decrypted-handle-${index.toString()}`}>
                                            {key}
                                        </span>
                                        :
                                        <span data-testid={`public-decrypt-decrypted-value-${index.toString()}`}>
                                            {value}
                                        </span>
                                    </code>
                                </li>
                            ))}
                        </ul>
                        <p className="muted small">ABI-encoded clear values</p>
                        <code className="code-block">{publicDecryptSummary.abiEncodedClearValues}</code>
                        <p className="muted small">Decryption proof</p>
                        <code className="code-block">{publicDecryptSummary.decryptionProof}</code>
                    </article>
                </div>
            ) : null}

            <LogPanel lines={logLines} onClear={clearLog} />
        </section>
    )
}
