// import type { FhevmInstance, ZKProofLike } from "@zama-fhe/relayer-sdk/bundle"
// import { createInstance, initSDK } from "@zama-fhe/relayer-sdk/bundle"
import { useCallback, useEffect, useMemo, useState } from "react"
import {
    // type AbiFunction,
    type Hex,
    // createWalletClient,
    // http,
    // publicActions,
} from "viem"
import { type Account } from "viem/accounts"

// import { sepolia } from "viem/chains"

// import InputVerifier from "../assets/InputVerifier.json"
import { loadWalletAccount } from "../utils/account"
import { type FhevmChainName, credentials, resolveFhevmChainConfig } from "../utils/const"
import { formatDuration, formatError /* , toHexString */ } from "../utils/fmt"
// import {
//     randomAddress,
//     randomUint8,
//     randomUint16,
//     randomUint32,
//     randomUint64,
//     randomUint128,
//     randomUint256,
// } from "../utils/random"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import type {
    CiphertextSummary,
    // EncryptedInputBuffer,
    InputSummary,
    VerifyInputSummary,
} from "../utils/types"
import { Config } from "./config"
import "./route.css"

// const client = createWalletClient({
//     chain: sepolia,
//     transport: http(import.meta.env.VITE_SEPOLIA_RPC_URL as string),
// }).extend(publicActions)

// const verifyInputAbi: AbiFunction = {
//     inputs: [
//         {
//             components: [
//                 {
//                     internalType: "address",
//                     name: "userAddress",
//                     type: "address",
//                 },
//                 {
//                     internalType: "address",
//                     name: "contractAddress",
//                     type: "address",
//                 },
//             ],
//             internalType: "struct FHEVMExecutor.ContextUserInputs",
//             name: "context",
//             type: "tuple",
//         },
//         {
//             internalType: "bytes32",
//             name: "inputHandle",
//             type: "bytes32",
//         },
//         {
//             internalType: "bytes",
//             name: "inputProof",
//             type: "bytes",
//         },
//     ],
//     name: "verifyInput",
//     outputs: [
//         {
//             internalType: "bytes32",
//             name: "",
//             type: "bytes32",
//         },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
// }

// const inputVerifierAbi = InputVerifier.abi

//const dummyContractAddress: Hex = randomAddress()

interface VerifyInputProps {
    config: FhevmChainName
}

/**
 * React Component to test the verification of an input proof of mixed FHEVM handles with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const VerifyInput = ({ config }: VerifyInputProps) => {
    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [accountState, setAccountState] = useState<AsyncState>("idle")
    const [inputState, setInputState] = useState<AsyncState>("idle")
    const [bufferState, setBufferState] = useState<AsyncState>("idle")
    const [zkProofState, setZKProofState] = useState<AsyncState>("idle")
    const [verifZKPState, setVerifZKPState] = useState<AsyncState>("idle")
    const [verifyInputState, setVerifyInputState] = useState<AsyncState>("idle")
    const [accountAddress, setAccountAddress] = useState<Hex | null>(null)
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [inputSummary, setInputSummary] = useState<InputSummary | null>(null)
    const [ciphertextSummary, setCiphertextSummary] = useState<CiphertextSummary | null>(null)
    const [verifyInputSummary, setVerifyInputSummary] = useState<VerifyInputSummary | null>(null)
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
        loadAccount?: number
        input?: number
        buffer?: number
        zkproof?: number
        verifyZkProof?: number
        verifyInputState?: number
    }>({})

    const isRunning =
        initState === "pending" ||
        instanceState === "pending" ||
        accountState === "pending" ||
        inputState === "pending" ||
        bufferState === "pending" ||
        zkProofState === "pending" ||
        verifZKPState === "pending" ||
        verifyInputState === "pending"
    const relayerConfig = useMemo(() => resolveFhevmChainConfig(config), [config])

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setAccountAddress(null)
        setInputSummary(null)
        setCiphertextSummary(null)
        setVerifyInputSummary(null)
        setDurations({}) // Reset durations on retry

        setInitState("pending")
        setInstanceState("idle")
        setAccountState("idle")
        setInputState("idle")
        setBufferState("idle")
        setZKProofState("idle")
        setVerifZKPState("idle")
        setVerifyInputState("idle")
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

    const createRelayerInstance = useCallback(
        async () => {
            setInstanceState("pending")
            await nextTick()

            const start = performance.now()
            // const instance = await createInstance(relayerConfig)
            console.log("createRelayerInstance")
            const instance = null
            const end = performance.now()

            setDurations((prev) => ({ ...prev, instance: end - start }))
            setInstanceState("success")
            return instance
        },
        [
            /* relayerConfig */
        ]
    )

    const loadAccount = useCallback(async () => {
        setAccountState("pending")
        await nextTick()

        const start = performance.now()
        const account = loadWalletAccount(credentials)
        const end = performance.now()

        setAccountAddress(account.address)
        setDurations((prev) => ({ ...prev, loadAccount: end - start }))
        setAccountState("success")

        return account
    }, [])

    const createInputBuffer = useCallback(async (/* instance: FhevmInstance, */ userAddress: Hex) => {
        setInputState("pending")
        await nextTick()

        const start = performance.now()
        // const buffer = instance.createEncryptedInput(
        //     dummyContractAddress,
        //     userAddress
        // )
        console.log("createEncryptedInput " + userAddress)
        const buffer = null
        const end = performance.now()

        setDurations((prev) => ({ ...prev, input: end - start }))
        setInputState("success")
        return buffer
    }, [])

    const populateInputBuffer = useCallback(async (/* buffer: EncryptedInputBuffer, */ userAddress: Hex) => {
        setBufferState("pending")
        await nextTick()
        // const sampleBool = randomUint8() % 2 === 0
        // const sampleUint8 = randomUint8()
        // const sampleUint16 = randomUint16()
        // const sampleUint32 = randomUint32()
        // const sampleUint64 = randomUint64()
        // const sampleUint128 = randomUint128()
        // const sampleUint256 = randomUint256()
        // const sampleAddress = randomAddress()

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
        // const values = [
        //     { label: "addBool", value: sampleBool.toString() },
        //     { label: "add8", value: sampleUint8.toString() },
        //     { label: "add16", value: sampleUint16.toString() },
        //     { label: "add32", value: sampleUint32.toString() },
        //     { label: "add64", value: sampleUint64.toString() },
        //     { label: "add128", value: sampleUint128.toString() },
        //     { label: "add256", value: sampleUint256.toString() },
        //     {
        //         label: "addAddress",
        //         value: sampleAddress,
        //     },
        // ]

        // setInputSummary({
        //     contractAddress: dummyContractAddress,
        //     userAddress,
        //     // bits: [...bits],
        //     values,
        // })
        setDurations((prev) => ({ ...prev, buffer: end - start }))
        setBufferState("success")
    }, [])

    const generateZKProof = useCallback(
        async (/* buffer: EncryptedInputBuffer */) => {
            setZKProofState("pending")
            await nextTick()

            const start = performance.now()
            // const zkproof: ZKProofLike = buffer.generateZKProof()
            console.log("generateZKProof")
            const zkproof = null
            const end = performance.now()

            setDurations((prev) => ({ ...prev, zkproof: end - start }))
            setZKProofState("success")

            return zkproof
        },
        []
    )

    const requestZKPVerif = useCallback(
        async (/* instance: FhevmInstance, zkproof: ZKProofLike */) => {
            setVerifZKPState("pending")
            await nextTick()

            const start = performance.now()
            // const ciphertexts =
            //     await instance.requestZKProofVerification(zkproof)
            console.log("requestZKPVerif")
            const end = performance.now()

            const summary = {
                handles: [] as string[],
                inputProof: "0x" as string,
            }
            setCiphertextSummary(summary)
            setDurations((prev) => ({ ...prev, verifyZkProof: end - start }))
            setVerifZKPState("success")

            return summary
        },
        []
    )

    const verifyInput = useCallback(
        async (_account: Account, _contractAddress: Hex, _ciphertexts: Hex[], _inputProof: Hex) => {
            setVerifyInputState("pending")
            await nextTick()
            const results: string[] = []

            const start = performance.now()
            // for (const text of ciphertexts) {
            //     const { result } = await client.simulateContract({
            //         account,
            //         address: relayerConfig.inputVerifierContractAddress as Hex,
            //         abi: inputVerifierAbi,
            //         functionName: verifyInputAbi.name,
            //         args: [
            //             [account.address, contractAddress],
            //             text,
            //             inputProof,
            //         ],
            //     })
            //     results.push(result as string)
            // }
            console.log("verifyInput")
            const end = performance.now()

            setVerifyInputSummary({ results })
            setDurations((prev) => ({ ...prev, verifyInputState: end - start }))
            setVerifyInputState("success")
        },
        [
            /* relayerConfig */
        ]
    )

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initSdkStep()
            // const instance = await createRelayerInstance()
            // const loadedAccount = await loadAccount()
            // const buffer = await createInputBuffer(
            //     instance,
            //     loadedAccount.address
            // )
            // await populateInputBuffer(buffer, loadedAccount.address)
            // const zkproof = await generateZKProof(buffer)
            // const { handles, inputProof } = await requestZKPVerif(
            //     instance,
            //     zkproof
            // )
            // await verifyInput(
            //     loadedAccount,
            //     dummyContractAddress,
            //     handles,
            //     inputProof
            // )
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setAccountState((prev) => (prev === "pending" ? "error" : prev))
            setInputState((prev) => (prev === "pending" ? "error" : prev))
            setBufferState((prev) => (prev === "pending" ? "error" : prev))
            setZKProofState((prev) => (prev === "pending" ? "error" : prev))
            setVerifZKPState((prev) => (prev === "pending" ? "error" : prev))
            setVerifyInputState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`Input Verification flow failed: ${formatError(error)}`)
        }
    }, [
        createInputBuffer,
        createRelayerInstance,
        generateZKProof,
        requestZKPVerif,
        initSdkStep,
        loadAccount,
        populateInputBuffer,
        resetFlowState,
        verifyInput,
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
                <div className="status-error" role="alert" id="verify-input-flow-error">
                    <strong>Public decrypt flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <h2>Input Verification</h2>
                <Config fhevmChainConfig={relayerConfig}></Config>
                <button
                    id="verify-input-flow-trigger"
                    data-testid="verify-input-flow-trigger"
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
                            <span className="duration-badge">{formatDuration(durations.init)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(initState) ? (
                        <dd data-testid="verify-input-init-sdk-status-dd">{statusCopy[initState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={instanceState}>
                    <dt>
                        createInstance()
                        {durations.instance !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.instance)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(instanceState) ? (
                        <dd data-testid="verify-input-create-instance-status-dd">{statusCopy[instanceState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={accountState}>
                    <dt>
                        Load account
                        {durations.loadAccount !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.loadAccount)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(accountState) ? (
                        <dd data-testid="verify-input-load-account-status-dd">{statusCopy[accountState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={inputState}>
                    <dt>
                        createEncryptedInput()
                        {durations.input !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.input)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(inputState) ? (
                        <dd data-testid="verify-input-create-input-status-dd">{statusCopy[inputState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={bufferState}>
                    <dt>
                        Populate buffer
                        {durations.buffer !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.buffer)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(bufferState) ? (
                        <dd data-testid="verify-input-populate-buffer-status-dd">{statusCopy[bufferState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={zkProofState}>
                    <dt>
                        Generate ZK Proof
                        {durations.zkproof !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.zkproof)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(zkProofState) ? (
                        <dd data-testid="verify-input-run-generate-zk-proof-status-dd">{statusCopy[zkProofState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={verifZKPState}>
                    <dt>
                        Verify ZK Proof - Relayer
                        {durations.verifyZkProof !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.verifyZkProof)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(verifZKPState) ? (
                        <dd data-testid="verify-input-run-verif-zkp-status-dd">{statusCopy[verifZKPState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={verifyInputState}>
                    <dt>
                        InputVerifier.verifyInput()
                        {durations.verifyInputState !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.verifyInputState)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(verifyInputState) ? (
                        <dd data-testid="verify-input-run-verify-input-status-dd">{statusCopy[verifyInputState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {formattedCompletedAt ? <p className="muted timestamp">Flow completed at {formattedCompletedAt}</p> : null}

            {accountAddress || inputSummary || ciphertextSummary || verifyInputSummary ? (
                <div className="result-grid">
                    {accountAddress ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Account</h3>
                                <span className="badge">ready</span>
                            </div>
                            <p className="muted small">Address</p>
                            <code className="code-block">{accountAddress}</code>
                        </article>
                    ) : null}

                    {inputSummary ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Encrypted input</h3>
                                <span className="badge">created</span>
                            </div>
                            <ul className="data-list">
                                <li>
                                    <span className="muted small">Contract address</span>
                                    <code className="code-block">{inputSummary.contractAddress}</code>
                                </li>
                                <li>
                                    <span className="muted small">User address (wallet)</span>
                                    <code className="code-block">{inputSummary.userAddress}</code>
                                </li>
                                <li>
                                    <span className="muted small">Added values</span>
                                    <ul className="code-stack">
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
                                    <span className="muted small">Bit-length plan</span>
                                    <div className="chip-row">
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
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Ciphertexts</h3>
                                <span className="badge">created</span>
                            </div>
                            <p className="muted small">Handles ({ciphertextSummary.handles.length})</p>
                            <ul className="code-stack">
                                {ciphertextSummary.handles.map((handle, index) => (
                                    <li key={`${handle}-${index.toString()}`}>
                                        <code
                                            className="code-block"
                                            data-testid={`verify-input-ciphertext-handle-${index.toString()}`}
                                        >
                                            {handle}
                                        </code>
                                    </li>
                                ))}
                            </ul>
                            <p className="muted small">Input proof</p>
                            <code
                                className="code-block"
                                data-testid="verify-input-ciphertext-proof"
                                id="verify-input-ciphertext-proof"
                            >
                                {ciphertextSummary.inputProof}
                            </code>
                        </article>
                    ) : null}

                    {verifyInputSummary ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Input Verification results</h3>
                                <span className="badge">verified</span>
                            </div>
                            <p className="muted small">Results ({verifyInputSummary.results.length})</p>
                            <ul className="code-stack">
                                {Object.entries(verifyInputSummary.results).map(([key, value], index) => (
                                    <li key={key}>
                                        <code
                                            className="code-block"
                                            data-testid={`verify-input-verified-ciphertext-handle-${index.toString()}`}
                                        >
                                            {value}
                                        </code>
                                    </li>
                                ))}
                            </ul>
                        </article>
                    ) : null}
                </div>
            ) : null}
        </section>
    )
}
