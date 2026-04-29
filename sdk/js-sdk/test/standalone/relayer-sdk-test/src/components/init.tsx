import { useCallback, useEffect, useMemo, useState } from "react"

import { createFullClient, initEncryptRuntime } from "../utils/client"
import { type FhevmChainName, resolveFhevmChainConfig } from "../utils/const"
import { formatDuration, formatError } from "../utils/fmt"
import { LogPanel, useLog } from "../utils/log"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import { Config } from "./config"
import "./route.css"

interface InitProps {
    config: FhevmChainName
}

/**
 * React Component to test `initSDK()` and `createInstance()` of @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const Init = ({ config }: InitProps) => {
    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [instanceSummary, setInstanceSummary] = useState<string>("")
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
    }>({})

    ////////////////////////////////////////////////////////////////////////////
    // Misc
    ////////////////////////////////////////////////////////////////////////////

    const { lines: logLines, log, clear: clearLog } = useLog()

    const isRunning = initState === "pending" || instanceState === "pending"
    const relayerConfig = useMemo(() => resolveFhevmChainConfig(config), [config])

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setInstanceSummary("")
        setDurations({}) // Reset durations on retry
        clearLog()

        setInitState("pending")
        setInstanceState("idle")
    }, [clearLog])

    const initRuntime = useCallback(async () => {
        setInitState("pending")
        await nextTick()

        const start = performance.now()
        await initEncryptRuntime(relayerConfig, log)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, init: end - start }))
        setInitState("success")
    }, [relayerConfig])

    const createClient = useCallback(async () => {
        setInstanceState("pending")
        await nextTick()

        const start = performance.now()
        const fhevmClient = await createFullClient(relayerConfig)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, instance: end - start }))
        setInstanceState("success")
        return fhevmClient
    }, [relayerConfig])

    const runFlow = useCallback(async () => {
        resetFlowState()
        log("--- Flow started ---")

        try {
            await initRuntime()
            await createClient()
            setCompletedAt(new Date().toISOString())
            log("--- Flow completed ---")
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            const msg = formatError(error)
            setErrorMessage(`Encryption flow failed: ${msg}`)
            log(`ERROR: ${msg}`)
        }
    }, [initRuntime, createClient, resetFlowState, log])

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

    return (
        <section className="panel">
            {errorMessage ? (
                <div className="status-error">
                    <strong>Initialization error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <h2>SDK Initialization</h2>
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
                        <dd data-testid="init-sdk-status-dd">{statusCopy[initState]}</dd>
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
                        <dd data-testid="create-instance-status-dd">{statusCopy[instanceState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {completedAt ? (
                <p className="muted timestamp">Completed at {new Date(completedAt).toLocaleTimeString()}</p>
            ) : null}

            {instanceSummary ? (
                <div className="status-output">
                    <h3>Instance summary</h3>
                    <p>{instanceSummary}</p>
                </div>
            ) : null}

            <LogPanel lines={logLines} onClear={clearLog} />
        </section>
    )
}
