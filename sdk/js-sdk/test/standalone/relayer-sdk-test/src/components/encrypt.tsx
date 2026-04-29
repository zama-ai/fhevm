import { useCallback, useEffect, useMemo, useState } from "react"
import { type Hex } from "viem"

import {
    createEncryptClient,
    encryptRandom,
    initEncryptRuntime,
} from "../utils/client"
import { type FhevmChainName, resolveFhevmChainConfig } from "../utils/const"
import { formatDuration, formatError } from "../utils/fmt"
import { LogPanel, useLog } from "../utils/log"
import { randomAddress } from "../utils/random"
import {
    type AsyncState,
    nextTick,
    shouldDisplayStatus,
    statusCopy,
} from "../utils/state"
import type {
    CiphertextSummary,
    EncryptClient,
    InputSummary,
} from "../utils/types"
import { Config } from "./config"
import "./route.css"

const dummyContractAddress: Hex = randomAddress()
const dummyUserAddress: Hex = randomAddress()

interface EncryptProps {
    config: FhevmChainName
}

/**
 * React Component to test encryption of supported FHEVM handles with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const Encrypt = ({ config }: EncryptProps) => {
    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [encryptState, setEncryptState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [inputSummary, setInputSummary] = useState<InputSummary | null>(null)
    const [ciphertextSummary, setCiphertextSummary] =
        useState<CiphertextSummary | null>(null)
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
        input?: number
        buffer?: number
        encrypt?: number
    }>({})
    const { lines: logLines, log, clear: clearLog } = useLog()

    const isRunning =
        initState === "pending" ||
        instanceState === "pending" ||
        encryptState === "pending"
    const relayerConfig = useMemo(
        () => resolveFhevmChainConfig(config),
        [config]
    )

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setInputSummary(null)
        setCiphertextSummary(null)
        setDurations({}) // Reset durations on retry

        setInitState("idle")
        setInstanceState("idle")
        setEncryptState("idle")
    }, [])

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
        const fhevmClient = await createEncryptClient(relayerConfig)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, instance: end - start }))
        setInstanceState("success")
        return fhevmClient
    }, [relayerConfig])

    const encryptValues = useCallback(
        async (
            client: EncryptClient,
            userAddress: Hex,
            contractAddress: Hex
        ) => {
            setEncryptState("pending")
            await nextTick()

            const start = performance.now()
            const result = await encryptRandom({
                userAddress,
                contractAddress,
                client,
            })
            const end = performance.now()

            setInputSummary(result.inputSummary)
            setCiphertextSummary({
                handles: result.encryptedValues,
                inputProof: result.inputProof,
            })
            setDurations((prev) => ({ ...prev, buffer: end - start }))
            setEncryptState("success")
        },
        []
    )

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initRuntime()
            const client = await createClient()
            await encryptValues(client, dummyUserAddress, dummyContractAddress)
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setEncryptState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`Encryption flow failed: ${formatError(error)}`)
        }
    }, [initRuntime, createClient, encryptValues, resetFlowState])

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

    return (
        <section className="panel">
            {errorMessage ? (
                <div className="status-error">
                    <strong>Encryption flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <h2>Encryption</h2>
                <Config fhevmChainConfig={relayerConfig}></Config>
                <button
                    className="primary-btn"
                    type="button"
                    onClick={handleRetry}
                    disabled={isRunning}
                >
                    {buttonLabel}
                </button>
            </header>

            <dl className="status-grid">
                <div className="status-card" data-state={initState}>
                    <dt>
                        initRuntime()
                        {durations.init !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.init)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(initState) ? (
                        <dd data-testid="encrypt-init-sdk-status-dd">
                            {statusCopy[initState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={instanceState}>
                    <dt>
                        createClient()
                        {durations.instance !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.instance)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(instanceState) ? (
                        <dd data-testid="encrypt-create-instance-status-dd">
                            {statusCopy[instanceState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={encryptState}>
                    <dt>
                        encryptValues()
                        {durations.buffer !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.buffer)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(encryptState) ? (
                        <dd data-testid="encrypt-encrypt-status-dd">
                            {statusCopy[encryptState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {formattedCompletedAt ? (
                <p className="muted timestamp">
                    Flow completed at {formattedCompletedAt}
                </p>
            ) : null}

            {inputSummary || ciphertextSummary ? (
                <div className="result-grid">
                    <article className="result-card">
                        <div className="result-card__header">
                            <h3>Account</h3>
                            <span className="badge" data-testid="account-ready">
                                ready
                            </span>
                        </div>
                        <p className="muted small">Address</p>
                        <code className="code-block">{dummyUserAddress}</code>
                    </article>

                    {inputSummary ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Encrypted input</h3>
                                <span className="badge">created</span>
                            </div>
                            <ul className="data-list">
                                <li>
                                    <span className="muted small">
                                        Contract address
                                    </span>
                                    <code
                                        className="code-block"
                                        data-testid="encrypted-input-contract"
                                    >
                                        {inputSummary.contractAddress}
                                    </code>
                                </li>
                                <li>
                                    <span className="muted small">
                                        User address (wallet)
                                    </span>
                                    <code
                                        className="code-block"
                                        data-testid="encrypted-input-user"
                                    >
                                        {inputSummary.userAddress}
                                    </code>
                                </li>
                                <li>
                                    <span className="muted small">
                                        Added values
                                    </span>
                                    <ul
                                        className="code-stack"
                                        data-testid="encrypted-input-values"
                                    >
                                        {inputSummary.values.map((entry) => (
                                            <li key={entry.label}>
                                                <code className="code-block">
                                                    {entry.label}: {entry.value}
                                                </code>
                                            </li>
                                        ))}
                                    </ul>
                                </li>
                                <li>
                                    <span className="muted small">
                                        Bit-length plan
                                    </span>
                                    <div
                                        className="chip-row"
                                        data-testid="encrypted-input-bits"
                                    >
                                        {inputSummary.values.length ? (
                                            inputSummary.values.map((value) => (
                                                <span
                                                    className="chip"
                                                    key={value.type}
                                                >
                                                    {value.type}
                                                </span>
                                            ))
                                        ) : (
                                            <span className="chip muted">
                                                No values added yet
                                            </span>
                                        )}
                                    </div>
                                </li>
                            </ul>
                        </article>
                    ) : null}

                    {ciphertextSummary ? (
                        <article
                            className="result-card"
                            id="ciphertext-summary"
                        >
                            <div className="result-card__header">
                                <h3>Ciphertexts</h3>
                                <span
                                    className="badge"
                                    data-testid="ciphertext-ready"
                                >
                                    created
                                </span>
                            </div>
                            <p className="muted small">
                                Handles ({ciphertextSummary.handles.length})
                            </p>
                            <ul
                                className="code-stack"
                                data-testid="ciphertext-handles"
                            >
                                {ciphertextSummary.handles.map(
                                    (handle, index) => (
                                        <li
                                            key={`${handle}-${index.toString()}`}
                                        >
                                            <code className="code-block">
                                                {handle}
                                            </code>
                                        </li>
                                    )
                                )}
                            </ul>
                            <p className="muted small">Input proof</p>
                            <code
                                className="code-block"
                                data-testid="ciphertext-proof"
                                id="ciphertext-proof"
                            >
                                {ciphertextSummary.inputProof}
                            </code>
                        </article>
                    ) : null}
                </div>
            ) : null}

            <LogPanel lines={logLines} onClear={clearLog} />
        </section>
    )
}
