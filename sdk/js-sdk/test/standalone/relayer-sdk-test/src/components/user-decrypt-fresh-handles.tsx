import type { RelayerUserDecryptOptions, RelayerUserDecryptProgressArgs } from "@fhevm/sdk/actions/decrypt"
import type { TypedValue } from "@fhevm/sdk/types"
import { useCallback, useEffect, useMemo, useState } from "react"
import { type Account, type Hex, createWalletClient, http, publicActions } from "viem"
import { sepolia } from "viem/chains"

import { loadWalletAccount } from "../utils/account"
import { createFullClient, initFullRuntime } from "../utils/client"
import {
    type FhevmChainName,
    credentials,
    resolveFhevmChainConfig,
    resolveRelayerContractAddress,
} from "../utils/const"
import { DECRYPT_TYPE_LABELS, type DecryptType, bufferWriters, callMakeUserDecryptable } from "../utils/decrypt"
import { formatDuration, formatError, toUserDecryptTimestamp } from "../utils/fmt"
import { LogPanel, useLog } from "../utils/log"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import type { CiphertextSummary, DecryptClient, FullClient, InputSummary } from "../utils/types"
import { Config } from "./config"
import "./route.css"

interface UserDecryptProps {
    decryptType: DecryptType
    config: FhevmChainName
}

////////////////////////////////////////////////////////////////////////////////
// Wallet
////////////////////////////////////////////////////////////////////////////////

const walletClient = createWalletClient({
    chain: sepolia,
    transport: http(import.meta.env.VITE_SEPOLIA_RPC_URL as string),
}).extend(publicActions)

////////////////////////////////////////////////////////////////////////////////

/**
 * React Component to test user decryption of freshly user-encrypted values with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const UserDecryptFreshHandles = ({ decryptType, config }: UserDecryptProps) => {
    ////////////////////////////////////////////////////////////////////////////
    // States
    ////////////////////////////////////////////////////////////////////////////

    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [accountState, setAccountState] = useState<AsyncState>("idle")
    const [encryptState, setEncryptState] = useState<AsyncState>("idle")
    const [makeUserDecryptableState, setMakeUserDecryptableState] = useState<AsyncState>("idle")
    const [decryptState, setDecryptState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [accountAddress, setAccountAddress] = useState<string | null>(null)
    const [inputSummary, setInputSummary] = useState<InputSummary | null>(null)
    const [ciphertextSummary, setCiphertextSummary] = useState<CiphertextSummary | null>(null)
    // To be renamed
    const [userDecryptSummary, setUserDecryptSummary] = useState<readonly TypedValue[] | null>(null)
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
        loadAccount?: number
        encryptValues?: number
        makeUserDecryptable?: number
        decryptValues?: number
    }>({})

    ////////////////////////////////////////////////////////////////////////////
    // Misc
    ////////////////////////////////////////////////////////////////////////////

    const { lines: logLines, log, clear: clearLog } = useLog()

    const typeLabel = DECRYPT_TYPE_LABELS[decryptType]

    const isRunning =
        initState === "pending" ||
        instanceState === "pending" ||
        accountState === "pending" ||
        encryptState === "pending" ||
        makeUserDecryptableState === "pending" ||
        decryptState === "pending"

    const relayerConfig = useMemo(() => resolveFhevmChainConfig(config), [config])

    const relayerOptions: RelayerUserDecryptOptions | undefined = useMemo(() => {
        const option = {
            onProgress(args: RelayerUserDecryptProgressArgs) {
                log(`V2 ROUTE PROGRESS: ${args.type}`)
            },
        }
        return option
    }, [config])

    const relayerSDKTestContractAddress = useMemo(() => resolveRelayerContractAddress(config), [config])

    ////////////////////////////////////////////////////////////////////////////
    // Reset flow
    ////////////////////////////////////////////////////////////////////////////

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setAccountAddress(null)
        setInputSummary(null)
        setCiphertextSummary(null)
        setUserDecryptSummary(null)
        setDurations({}) // Reset durations on retry

        setInitState("pending")
        setInstanceState("idle")
        setAccountState("idle")
        setEncryptState("idle")
        setMakeUserDecryptableState("idle")
        setDecryptState("idle")
    }, [])

    ////////////////////////////////////////////////////////////////////////////
    // Step 1: initRuntime
    ////////////////////////////////////////////////////////////////////////////

    const initRuntime = useCallback(async () => {
        setInitState("pending")
        await nextTick()

        const start = performance.now()
        await initFullRuntime(relayerConfig, log)
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
        const fhevmClient = await createFullClient(relayerConfig)
        const end = performance.now()

        setDurations((prev) => ({ ...prev, instance: end - start }))
        setInstanceState("success")
        return fhevmClient
    }, [relayerConfig])

    ////////////////////////////////////////////////////////////////////////////
    // Step 3: loadAccount
    ////////////////////////////////////////////////////////////////////////////

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

    ////////////////////////////////////////////////////////////////////////////
    // Step 4: encryptValues
    ////////////////////////////////////////////////////////////////////////////

    const encryptValues = useCallback(
        async (client: FullClient, userAddress: Hex) => {
            setEncryptState("pending")
            await nextTick()

            const start = performance.now()
            const { values, valuesSummary } = bufferWriters[decryptType]()
            const ciphertexts = await client.encryptValues({
                values,
                contractAddress: relayerSDKTestContractAddress,
                userAddress,
            })
            const end = performance.now()

            setInputSummary({
                contractAddress: relayerSDKTestContractAddress,
                userAddress,
                values: valuesSummary,
            })
            setCiphertextSummary({
                handles: ciphertexts.encryptedValues,
                inputProof: ciphertexts.inputProof,
            })
            setDurations((prev) => ({ ...prev, encryptValues: end - start }))
            setEncryptState("success")

            return ciphertexts
        },
        [decryptType, relayerSDKTestContractAddress]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Step 5: makeUserDecryptable
    ////////////////////////////////////////////////////////////////////////////

    const makeUserDecryptable = useCallback(
        async (account: Account, ciphertexts: Hex | readonly Hex[], inputProof: Hex) => {
            setMakeUserDecryptableState("pending")
            await nextTick()

            const start = performance.now()
            const xValues: Hex[] = await callMakeUserDecryptable({
                account,
                ciphertexts,
                inputProof,
                contractAddress: relayerSDKTestContractAddress,
                decryptType,
                walletClient,
            })
            const end = performance.now()
            setDurations((prev) => ({
                ...prev,
                makeUserDecryptable: end - start,
            }))
            setMakeUserDecryptableState("success")

            return xValues
        },
        [decryptType, relayerSDKTestContractAddress]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Step 6: userDecryptValue
    ////////////////////////////////////////////////////////////////////////////

    const userDecryptValue = useCallback(
        async (client: DecryptClient, account: Account, xValues: Hex[], contractAddress: Hex) => {
            setDecryptState("pending")

            await nextTick()

            const transportKeyPair = await client.generateTransportKeyPair()
            const startTimestamp = toUserDecryptTimestamp()
            const durationDays = 1

            const signedPermit = await client.signDecryptionPermit({
                contractAddresses: [contractAddress],
                durationDays,
                startTimestamp,
                signerAddress: account.address,
                signer: account,
                transportKeyPair,
            })

            const start = performance.now()

            const decrypted: readonly TypedValue[] = await client.decryptValues({
                encryptedValues: xValues,
                contractAddress,
                signedPermit,
                transportKeyPair,
                options: relayerOptions,
            })

            const end = performance.now()

            setUserDecryptSummary(decrypted)
            setDurations((prev) => ({ ...prev, decryptValues: end - start }))

            setUserDecryptSummary(null)
            setDecryptState("success")
        },
        [relayerSDKTestContractAddress /*, relayerOptions */]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Run Flow
    ////////////////////////////////////////////////////////////////////////////

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initRuntime()
            const fhevmClient = await createClient()
            const account = await loadAccount()
            await encryptValues(fhevmClient, account.address)
            const ciphertexts = await encryptValues(fhevmClient, account.address)
            const handles = await makeUserDecryptable(account, ciphertexts.encryptedValues, ciphertexts.inputProof)
            await userDecryptValue(fhevmClient, account, handles, relayerSDKTestContractAddress)
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setAccountState((prev) => (prev === "pending" ? "error" : prev))
            setEncryptState((prev) => (prev === "pending" ? "error" : prev))
            setMakeUserDecryptableState((prev) => (prev === "pending" ? "error" : prev))
            setDecryptState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`${decryptType} user decryption flow failed: ${formatError(error)}`)
        }
    }, [
        initRuntime,
        createClient,
        loadAccount,
        makeUserDecryptable,
        userDecryptValue,
        resetFlowState,
        decryptType,
        relayerSDKTestContractAddress,
    ])

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
                    <strong>User decrypt flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <div>
                    <h2>User decryption</h2>
                    <div className="chip-row">
                        <span className="chip" data-testid="user-decrypt-selected-type">
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
                        <dd data-testid="user-decrypt-init-sdk-status-dd">{statusCopy[initState]}</dd>
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
                        <dd data-testid="user-decrypt-create-instance-status-dd">{statusCopy[instanceState]}</dd>
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
                        <dd data-testid="user-decrypt-load-account-status-dd">{statusCopy[accountState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={encryptState}>
                    <dt>
                        encryptValues()
                        {durations.encryptValues !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.encryptValues)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(encryptState) ? (
                        <dd data-testid="user-decrypt-encrypt-status-dd">{statusCopy[encryptState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={makeUserDecryptableState}>
                    <dt>
                        makeUserDecryptable()
                        {durations.makeUserDecryptable !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.makeUserDecryptable)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(makeUserDecryptableState) ? (
                        <dd data-testid="user-decrypt-run-make-user-decryptable-status-dd">
                            {statusCopy[makeUserDecryptableState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={decryptState}>
                    <dt>
                        decryptValues()
                        {durations.decryptValues !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.decryptValues)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(decryptState) ? (
                        <dd data-testid="user-decrypt-run-decryption-status-dd">{statusCopy[decryptState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {formattedCompletedAt ? <p className="muted timestamp">Flow completed at {formattedCompletedAt}</p> : null}

            {accountAddress || inputSummary || ciphertextSummary || userDecryptSummary ? (
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
                                    <span className="muted small">Value type</span>
                                    <code className="code-block">{typeLabel}</code>
                                </li>
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
                                        {inputSummary.values.map((entry, index) => (
                                            <li key={entry.label}>
                                                <code className="code-block">
                                                    {entry.label}:
                                                    <span data-testid={`user-decrypt-input-values-${index.toString()}`}>
                                                        {entry.value}
                                                    </span>
                                                </code>
                                            </li>
                                        ))}
                                    </ul>
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
                                            data-testid={`user-decrypt-ciphertext-handles-${index.toString()}`}
                                        >
                                            {handle}
                                        </code>
                                    </li>
                                ))}
                            </ul>
                            <p className="muted small">Input proof</p>
                            <code className="code-block">{ciphertextSummary.inputProof}</code>
                        </article>
                    ) : null}

                    {userDecryptSummary ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>User decrypted values</h3>
                                <span className="badge">decrypted</span>
                            </div>
                            <p className="muted small">Clear values</p>
                            <ul className="code-stack">
                                {Object.entries(userDecryptSummary).map(([key, value], index) => (
                                    <li key={key}>
                                        <code className="code-block">
                                            <span data-testid={`user-decrypt-decrypted-handle-${index.toString()}`}>
                                                {key}
                                            </span>
                                            :
                                            <span data-testid={`user-decrypt-decrypted-value-${index.toString()}`}>
                                                {String(value)}
                                            </span>
                                        </code>
                                    </li>
                                ))}
                            </ul>
                        </article>
                    ) : null}
                </div>
            ) : null}
            <LogPanel lines={logLines} onClear={clearLog} />
        </section>
    )
}
