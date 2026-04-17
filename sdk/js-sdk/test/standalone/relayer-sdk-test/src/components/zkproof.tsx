// import type { FhevmInstance, ZKProofLike } from "@zama-fhe/relayer-sdk/bundle"
// import { createInstance, initSDK } from "@zama-fhe/relayer-sdk/bundle"
import { useCallback, useEffect, useMemo, useState } from "react"
import type { Hex } from "viem"

import { type FhevmChainName, resolveFhevmChainConfig } from "../utils/const"
import { formatDuration, formatError } from "../utils/fmt"
import {
    randomAddress,
    randomUint8,
    randomUint16,
    randomUint32,
    randomUint64,
    randomUint128,
    randomUint256,
} from "../utils/random"
import {
    type AsyncState,
    nextTick,
    shouldDisplayStatus,
    statusCopy,
} from "../utils/state"
import type {
    CiphertextSummary,
    //EncryptedInputBuffer,
    InputSummary,
} from "../utils/types"
import { Config } from "./config"
import "./route.css"

const dummyContractAddress: Hex = randomAddress()
const dummyUserAddress: Hex = randomAddress()

interface ZKProofProps {
    config: FhevmChainName
}

/**
 * React Component to test ZKProof generation of supported FHEVM handles with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const ZKProof = ({ config }: ZKProofProps) => {
    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [inputState, setInputState] = useState<AsyncState>("idle")
    const [bufferState, setBufferState] = useState<AsyncState>("idle")
    const [zkProofState, setZKProofState] = useState<AsyncState>("idle")
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
        zkproof?: number
    }>({})

    const isRunning =
        initState === "pending" ||
        instanceState === "pending" ||
        inputState === "pending" ||
        bufferState === "pending" ||
        zkProofState === "pending"
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
        setInputState("idle")
        setBufferState("idle")
        setZKProofState("idle")
    }, [])

    const initSdkStep = useCallback(async () => {
        setInitState("pending")
        await nextTick()

        const start = performance.now()
        // await initSDK({ thread: undefined })
        console.log("initSdkStep")
        const end = performance.now()

        setDurations((prev) => ({ ...prev, init: end - start }))
        setInitState("success")
    }, [])

    const createRelayerInstance = useCallback(async () => {
        setInstanceState("pending")
        await nextTick()

        const start = performance.now()
        // const instance = await createInstance(relayerConfig)
        console.log("createRelayerInstance")
        const end = performance.now()

        setDurations((prev) => ({ ...prev, instance: end - start }))
        setInstanceState("success")
        return null
    }, [relayerConfig])

    const createInputBuffer = useCallback(
        async (/*instance: FhevmInstance,*/ userAddress: Hex) => {
            setInputState("pending")
            await nextTick()

            const start = performance.now()
            // const buffer = instance.createEncryptedInput(
            //     dummyContractAddress,
            //     userAddress
            // )
            console.log("createEncryptedInput " + userAddress)
            const end = performance.now()

            setDurations((prev) => ({ ...prev, input: end - start }))
            setInputState("success")
            return null
        },
        []
    )

    const populateInputBuffer = useCallback(
        async (/*buffer: EncryptedInputBuffer,*/ userAddress: Hex) => {
            setBufferState("pending")
            await nextTick()
            const sampleBool = randomUint8() % 2 === 0
            const sampleUint8 = randomUint8()
            const sampleUint16 = randomUint16()
            const sampleUint32 = randomUint32()
            const sampleUint64 = randomUint64()
            const sampleUint128 = randomUint128()
            const sampleUint256 = randomUint256()
            const sampleAddress = randomAddress()

            const start = performance.now()
            // buffer.addBool(sampleBool)
            // buffer.add8(sampleUint8)
            // buffer.add16(sampleUint16)
            // buffer.add32(sampleUint32)
            // buffer.add64(sampleUint64)
            // buffer.add128(sampleUint128)
            // buffer.add256(sampleUint256)
            // buffer.addAddress(sampleAddress)
            console.log("populateInputBuffer " + userAddress)
            const end = performance.now()

            // const bits = buffer.getBits()
            const values = [
                {
                    label: "addBool",
                    value: sampleBool.toString(),
                    type: "ebool",
                },
                {
                    label: "add8",
                    value: sampleUint8.toString(),
                    type: "euint8",
                },
                {
                    label: "add16",
                    value: sampleUint16.toString(),
                    type: "euint16",
                },
                {
                    label: "add32",
                    value: sampleUint32.toString(),
                    type: "euint32",
                },
                {
                    label: "add64",
                    value: sampleUint64.toString(),
                    type: "euint64",
                },
                {
                    label: "add128",
                    value: sampleUint128.toString(),
                    type: "euint128",
                },
                {
                    label: "add256",
                    value: sampleUint256.toString(),
                    type: "euint256",
                },
                {
                    label: "addAddress",
                    value: sampleAddress,
                    type: "eaddress",
                },
            ]

            setInputSummary({
                contractAddress: dummyContractAddress,
                userAddress,
                values,
            })
            setDurations((prev) => ({ ...prev, buffer: end - start }))
            setBufferState("success")
        },
        []
    )

    const generateZKProof = useCallback(
        async (/*buffer: EncryptedInputBuffer*/) => {
            setZKProofState("pending")
            await nextTick()

            const start = performance.now()
            // const zkproof: ZKProofLike = buffer.generateZKProof()
            console.log("generateZKProof")
            const end = performance.now()

            setDurations((prev) => ({ ...prev, zkproof: end - start }))
            setZKProofState("success")

            return null
        },
        []
    )

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initSdkStep()
            // const instance = await createRelayerInstance()
            // const buffer = await createInputBuffer(instance, dummyUserAddress)
            // await populateInputBuffer(buffer, dummyUserAddress)
            // await generateZKProof(buffer)
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setInputState((prev) => (prev === "pending" ? "error" : prev))
            setBufferState((prev) => (prev === "pending" ? "error" : prev))
            setZKProofState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`ZK Proof flow failed: ${formatError(error)}`)
        }
    }, [
        createInputBuffer,
        createRelayerInstance,
        generateZKProof,
        initSdkStep,
        populateInputBuffer,
        resetFlowState,
    ])

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
                    <strong>ZK Proof flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <h2>ZK Proof</h2>
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
                        initSDK()
                        {durations.init !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.init)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(initState) ? (
                        <dd data-testid="zk-proof-init-sdk-status-dd">
                            {statusCopy[initState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={instanceState}>
                    <dt>
                        createInstance()
                        {durations.instance !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.instance)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(instanceState) ? (
                        <dd data-testid="zk-proof-create-instance-status-dd">
                            {statusCopy[instanceState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={inputState}>
                    <dt>
                        createEncryptedInput() Buffer
                        {durations.input !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.input)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(inputState) ? (
                        <dd data-testid="zk-proof-create-input-status-dd">
                            {statusCopy[inputState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={bufferState}>
                    <dt>
                        Populate buffer
                        {durations.buffer !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.buffer)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(bufferState) ? (
                        <dd data-testid="zk-proof-populate-buffer-status-dd">
                            {statusCopy[bufferState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={zkProofState}>
                    <dt>
                        Generate ZK Proof
                        {durations.zkproof !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.zkproof)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(zkProofState) ? (
                        <dd data-testid="zk-proof-run-generate-zk-proof-status-dd">
                            {statusCopy[zkProofState]}
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
                                        {/* {inputSummary.bits.length ? (
                                            inputSummary.bits.map((bit) => (
                                                <span
                                                    className="chip"
                                                    key={bit}
                                                >
                                                    {bit} bits
                                                </span>
                                            ))
                                        ) : (
                                            <span className="chip muted">
                                                No values added yet
                                            </span>
                                        )} */}
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
        </section>
    )
}
