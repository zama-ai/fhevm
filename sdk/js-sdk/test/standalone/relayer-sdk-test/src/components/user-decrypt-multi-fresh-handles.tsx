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
    resolveRelayerContractMirrorAddress,
} from "../utils/const"
import { DECRYPT_TYPE_LABELS, type DecryptType, bufferWriters, callMakeUserDecryptable } from "../utils/decrypt"
import { formatDuration, formatError, toUserDecryptTimestamp } from "../utils/fmt"
import { LogPanel, useLog } from "../utils/log"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import type { CiphertextSummary, DecryptClient, FullClient, InputSummary } from "../utils/types"
import { Config } from "./config"
import "./route.css"

interface UserDecryptMultiProps {
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
 * React Component to test user decryption of freshly user-encrypted values across multiple contracts
 * with @fhevm/sdk on a web browser.
 * @returns React Component
 */
export const UserDecryptMultiFreshHandles = ({ decryptType, config }: UserDecryptMultiProps) => {
    ////////////////////////////////////////////////////////////////////////////
    // States
    ////////////////////////////////////////////////////////////////////////////

    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [accountState, setAccountState] = useState<AsyncState>("idle")
    const [encryptState, setEncryptState] = useState<AsyncState>("idle")
    const [encryptMirrorState, setEncryptMirrorState] = useState<AsyncState>("idle")
    const [makeUserDecryptableState, setMakeUserDecryptableState] = useState<AsyncState>("idle")
    const [makeUserDecryptableMirrorState, setMakeUserDecryptableMirrorState] = useState<AsyncState>("idle")
    const [decryptState, setDecryptState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [accountAddress, setAccountAddress] = useState<string | null>(null)
    const [inputSummaries, setInputSummaries] = useState<InputSummary[]>([])
    const [ciphertextSummaries, setCiphertextSummaries] = useState<CiphertextSummary[]>([])
    const [userDecryptSummary, setUserDecryptSummary] = useState<readonly TypedValue[] | null>(null)
    const [completedAt, setCompletedAt] = useState<string | null>(null)
    const [attempts, setAttempts] = useState(0)
    const [durations, setDurations] = useState<{
        init?: number
        instance?: number
        loadAccount?: number
        encryptValues?: number
        encryptValuesMirror?: number
        makeUserDecryptable?: number
        makeUserDecryptableMirror?: number
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
        encryptMirrorState === "pending" ||
        makeUserDecryptableState === "pending" ||
        makeUserDecryptableMirrorState === "pending" ||
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
    const relayerSDKTestContractMirrorAddress = useMemo(() => resolveRelayerContractMirrorAddress(config), [config])

    ////////////////////////////////////////////////////////////////////////////
    // Reset flow
    ////////////////////////////////////////////////////////////////////////////

    const resetFlowState = useCallback(() => {
        setAttempts((prev) => prev + 1)
        setErrorMessage(null)
        setCompletedAt(null)
        setAccountAddress(null)
        setInputSummaries([])
        setCiphertextSummaries([])
        setUserDecryptSummary(null)
        setDurations({})

        setInitState("pending")
        setInstanceState("idle")
        setAccountState("idle")
        setEncryptState("idle")
        setEncryptMirrorState("idle")
        setMakeUserDecryptableState("idle")
        setMakeUserDecryptableMirrorState("idle")
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

        setDurations((prev) => ({ ...prev, loadAccount: end - start }))
        setAccountAddress(account.address)
        setAccountState("success")

        return account
    }, [])

    ////////////////////////////////////////////////////////////////////////////
    // Step 4: encryptValues (primary)
    ////////////////////////////////////////////////////////////////////////////

    const encryptValues = useCallback(
        async (client: FullClient, userAddress: Hex, contractAddress: Hex, isMirror: boolean) => {
            if (isMirror) {
                setEncryptMirrorState("pending")
            } else {
                setEncryptState("pending")
            }
            await nextTick()

            const start = performance.now()
            const { values, valuesSummary } = bufferWriters[decryptType]()
            const ciphertexts = await client.encryptValues({
                values,
                contractAddress,
                userAddress,
            })
            const end = performance.now()

            const durationKey = isMirror ? "encryptValuesMirror" : "encryptValues"
            setDurations((prev) => ({ ...prev, [durationKey]: end - start }))
            if (isMirror) {
                setEncryptMirrorState("success")
            } else {
                setEncryptState("success")
            }

            return {
                ciphertexts,
                inputSummary: {
                    contractAddress,
                    userAddress,
                    values: valuesSummary,
                } satisfies InputSummary,
            }
        },
        [decryptType]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Step 5: makeUserDecryptable
    ////////////////////////////////////////////////////////////////////////////

    const makeUserDecryptable = useCallback(
        async (
            account: Account,
            ciphertexts: Hex | readonly Hex[],
            inputProof: Hex,
            contractAddress: Hex,
            isMirror: boolean
        ) => {
            if (isMirror) {
                setMakeUserDecryptableMirrorState("pending")
            } else {
                setMakeUserDecryptableState("pending")
            }
            await nextTick()

            const start = performance.now()
            const xValues: Hex[] = await callMakeUserDecryptable({
                account,
                ciphertexts,
                inputProof,
                contractAddress,
                decryptType,
                walletClient,
            })
            const end = performance.now()

            const durationKey = isMirror ? "makeUserDecryptableMirror" : "makeUserDecryptable"
            setDurations((prev) => ({ ...prev, [durationKey]: end - start }))
            if (isMirror) {
                setMakeUserDecryptableMirrorState("success")
            } else {
                setMakeUserDecryptableState("success")
            }

            return xValues
        },
        [decryptType]
    )

    ////////////////////////////////////////////////////////////////////////////
    // Step 6: userDecryptMultiValue
    ////////////////////////////////////////////////////////////////////////////

    const userDecryptMultiValue = useCallback(
        async (
            client: DecryptClient,
            account: Account,
            pairs: Array<{
                encryptedValue: Hex
                contractAddress: Hex
            }>,
            contractAddress: Hex,
            contractMirrorAddress: Hex
        ) => {
            setDecryptState("pending")
            await nextTick()

            const transportKeypair = await client.generateTransportKeypair()
            const startTimestamp = toUserDecryptTimestamp()
            const durationDays = 1

            const signedPermit = await client.signDecryptionPermit({
                contractAddresses: [contractAddress, contractMirrorAddress],
                durationDays,
                startTimestamp,
                signerAddress: account.address,
                signer: account,
                transportKeypair,
            })

            const start = performance.now()

            const decrypted: readonly TypedValue[] = await client.decryptValuesFromPairs({
                pairs,
                signedPermit,
                transportKeypair,
                options: relayerOptions,
            })

            const end = performance.now()

            setUserDecryptSummary(decrypted)
            setDurations((prev) => ({ ...prev, decryptValues: end - start }))
            setDecryptState("success")

            return decrypted
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
            const account = await loadAccount()

            // Encrypt for primary contract
            const primary = await encryptValues(fhevmClient, account.address, relayerSDKTestContractAddress, false)
            // Encrypt for mirror contract
            const mirror = await encryptValues(fhevmClient, account.address, relayerSDKTestContractMirrorAddress, true)

            setInputSummaries([primary.inputSummary, mirror.inputSummary])
            setCiphertextSummaries([
                { handles: primary.ciphertexts.encryptedValues, inputProof: primary.ciphertexts.inputProof },
                { handles: mirror.ciphertexts.encryptedValues, inputProof: mirror.ciphertexts.inputProof },
            ])

            // Make user-decryptable on both contracts
            const handlesPrimary = await makeUserDecryptable(
                account,
                primary.ciphertexts.encryptedValues,
                primary.ciphertexts.inputProof,
                relayerSDKTestContractAddress,
                false
            )
            const handlesMirror = await makeUserDecryptable(
                account,
                mirror.ciphertexts.encryptedValues,
                mirror.ciphertexts.inputProof,
                relayerSDKTestContractMirrorAddress,
                true
            )

            const pairsPrimary = handlesPrimary.map((xValue) => ({
                encryptedValue: xValue,
                contractAddress: relayerSDKTestContractAddress,
            }))
            const pairsMirror = handlesMirror.map((xValue) => ({
                encryptedValue: xValue,
                contractAddress: relayerSDKTestContractMirrorAddress,
            }))
            const pairs = pairsPrimary.concat(pairsMirror)

            // Decrypt across both contracts
            const decrypted: readonly TypedValue[] = await userDecryptMultiValue(
                fhevmClient,
                account,
                pairs,
                relayerSDKTestContractAddress,
                relayerSDKTestContractMirrorAddress
            )

            // Verify: decrypted clear values must match the original encrypted values
            const primaryCount = handlesPrimary.length
            const mirrorCount = handlesMirror.length
            const expectedPrimary = primary.inputSummary.values
            const expectedMirror = mirror.inputSummary.values

            for (let i = 0; i < primaryCount; i++) {
                const got = String(decrypted[i].value)
                const want = expectedPrimary[i].value
                if (got !== want) {
                    throw new Error(
                        `Primary mismatch at index ${i} (${expectedPrimary[i].type}): expected ${want}, got ${got}`
                    )
                }
            }
            for (let i = 0; i < mirrorCount; i++) {
                const got = String(decrypted[primaryCount + i].value)
                const want = expectedMirror[i].value
                if (got !== want) {
                    throw new Error(
                        `Mirror mismatch at index ${i} (${expectedMirror[i].type}): expected ${want}, got ${got}`
                    )
                }
            }

            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setAccountState((prev) => (prev === "pending" ? "error" : prev))
            setEncryptState((prev) => (prev === "pending" ? "error" : prev))
            setEncryptMirrorState((prev) => (prev === "pending" ? "error" : prev))
            setMakeUserDecryptableState((prev) => (prev === "pending" ? "error" : prev))
            setMakeUserDecryptableMirrorState((prev) => (prev === "pending" ? "error" : prev))
            setDecryptState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`${decryptType} user decryption multi flow failed: ${formatError(error)}`)
        }
    }, [
        resetFlowState,
        initRuntime,
        createClient,
        loadAccount,
        encryptValues,
        makeUserDecryptable,
        userDecryptMultiValue,
        relayerSDKTestContractAddress,
        relayerSDKTestContractMirrorAddress,
        decryptType,
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
                    <strong>User decrypt multi flow error:</strong> {errorMessage}
                </div>
            ) : null}
            <header className="panel__header">
                <div>
                    <h2>User decryption Multi</h2>
                    <div className="chip-row">
                        <span className="chip" data-testid="user-decrypt-multi-selected-type">
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
                        <dd data-testid="user-decrypt-multi-init-sdk-status-dd">{statusCopy[initState]}</dd>
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
                        <dd data-testid="user-decrypt-multi-create-instance-status-dd">{statusCopy[instanceState]}</dd>
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
                        <dd data-testid="user-decrypt-multi-load-account-status-dd">{statusCopy[accountState]}</dd>
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
                        <dd data-testid="user-decrypt-multi-encrypt-status-dd">{statusCopy[encryptState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={encryptMirrorState}>
                    <dt>
                        encryptValues() (mirror)
                        {durations.encryptValuesMirror !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.encryptValuesMirror)}</span>
                        )}
                    </dt>
                    {shouldDisplayStatus(encryptMirrorState) ? (
                        <dd data-testid="user-decrypt-multi-encrypt-mirror-status-dd">
                            {statusCopy[encryptMirrorState]}
                        </dd>
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
                        <dd data-testid="user-decrypt-multi-run-make-user-decryptable-status-dd">
                            {statusCopy[makeUserDecryptableState]}
                        </dd>
                    ) : (
                        <></>
                    )}
                </div>
                <div className="status-card" data-state={makeUserDecryptableMirrorState}>
                    <dt>
                        makeUserDecryptable() (mirror)
                        {durations.makeUserDecryptableMirror !== undefined && (
                            <span className="duration-badge">
                                {formatDuration(durations.makeUserDecryptableMirror)}
                            </span>
                        )}
                    </dt>
                    {shouldDisplayStatus(makeUserDecryptableMirrorState) ? (
                        <dd data-testid="user-decrypt-multi-run-make-user-decryptable-mirror-status-dd">
                            {statusCopy[makeUserDecryptableMirrorState]}
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
                        <dd data-testid="user-decrypt-multi-run-decryption-status-dd">{statusCopy[decryptState]}</dd>
                    ) : (
                        <></>
                    )}
                </div>
            </dl>

            {formattedCompletedAt ? <p className="muted timestamp">Flow completed at {formattedCompletedAt}</p> : null}

            {accountAddress || inputSummaries.length || ciphertextSummaries.length || userDecryptSummary ? (
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

                    {inputSummaries.length ? (
                        <article className="result-card">
                            <div className="result-card__header">
                                <h3>Encrypted inputs</h3>
                                <span className="badge">created</span>
                            </div>
                            <ul className="data-list">
                                {inputSummaries.map((summary, index) => (
                                    <li key={`${summary.contractAddress}-${index.toString()}`}>
                                        <span className="muted small">Buffer {index + 1} contract</span>
                                        <code className="code-block">{summary.contractAddress}</code>
                                        <span className="muted small">User address (wallet)</span>
                                        <code className="code-block">{summary.userAddress}</code>
                                        <span className="muted small">Added values ({typeLabel})</span>
                                        <ul className="code-stack">
                                            {summary.values.map((entry, entryIndex) => (
                                                <li key={entry.label}>
                                                    <code className="code-block">
                                                        {entry.label}:{" "}
                                                        <span
                                                            data-testid={
                                                                index === 0
                                                                    ? `user-decrypt-multi-input-value-${entryIndex.toString()}`
                                                                    : `user-decrypt-multi-input-mirror-value-${entryIndex.toString()}`
                                                            }
                                                        >
                                                            {entry.value}
                                                        </span>
                                                    </code>
                                                </li>
                                            ))}
                                        </ul>
                                    </li>
                                ))}
                            </ul>
                        </article>
                    ) : null}

                    {ciphertextSummaries.length ? (
                        <article className="result-card" id="user-decrypt-multi-ciphertext-summary">
                            <div className="result-card__header">
                                <h3>Ciphertexts</h3>
                                <span className="badge" data-testid="user-decrypt-multi-ciphertext-ready">
                                    created
                                </span>
                            </div>
                            <ul className="data-list">
                                {ciphertextSummaries.map((summary, index) => (
                                    <li key={`${summary.inputProof}-${index.toString()}`}>
                                        <p className="muted small">Handles ({summary.handles.length})</p>
                                        <ul className="code-stack">
                                            {summary.handles.map((handle, handleIndex) => (
                                                <li key={`${handle}-${handleIndex.toString()}`}>
                                                    <code
                                                        className="code-block"
                                                        data-testid={
                                                            index === 0
                                                                ? `user-decrypt-multi-ciphertext-handle-${handleIndex.toString()}`
                                                                : `user-decrypt-multi-ciphertext-mirror-handle-${handleIndex.toString()}`
                                                        }
                                                    >
                                                        {handle}
                                                    </code>
                                                </li>
                                            ))}
                                        </ul>
                                        <p className="muted small">Input proof</p>
                                        <code className="code-block">{summary.inputProof}</code>
                                    </li>
                                ))}
                            </ul>
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
                                            <span
                                                data-testid={`user-decrypt-multi-decrypted-ciphertext-handle-${index.toString()}`}
                                            >
                                                {key}
                                            </span>
                                            :
                                            <span
                                                data-testid={`user-decrypt-multi-decrypted-value-${index.toString()}`}
                                            >
                                                {String(value.value)}
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
