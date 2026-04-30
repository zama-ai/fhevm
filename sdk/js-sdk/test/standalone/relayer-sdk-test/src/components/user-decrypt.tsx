import type { RelayerUserDecryptOptions, RelayerUserDecryptProgressArgs } from "@fhevm/sdk/actions/decrypt"
import type { TypedValue } from "@fhevm/sdk/types"
import { useCallback, useEffect, useMemo, useState } from "react"
import type { Account, Hex } from "viem"

import { loadWalletAccount } from "../utils/account"
import { createDecryptClient, initDecryptRuntime } from "../utils/client"
import {
    type FhevmChainName,
    credentials,
    resolveFhevmChainConfig,
    resolveRelayerContractAddress,
} from "../utils/const"
import { type DecryptType } from "../utils/decrypt"
import { formatDuration, formatError, toUserDecryptTimestamp } from "../utils/fmt"
import { getUserHandles } from "../utils/handles"
import { LogPanel, useLog } from "../utils/log"
import { type AsyncState, nextTick, shouldDisplayStatus, statusCopy } from "../utils/state"
import type { DecryptClient } from "../utils/types"
import { Config } from "./config"
import "./route.css"

interface UserDecryptProps {
    decryptType: DecryptType
    config: FhevmChainName
}

////////////////////////////////////////////////////////////////////////////////

/**
 * React Component to test user decryption of user-encrypted values with @zama-ai/relayer-sdk/bundle on a web browser.
 * @returns React Component
 */
export const UserDecrypt = ({ decryptType, config }: UserDecryptProps) => {
    ////////////////////////////////////////////////////////////////////////////
    // States
    ////////////////////////////////////////////////////////////////////////////

    const [initState, setInitState] = useState<AsyncState>("idle")
    const [instanceState, setInstanceState] = useState<AsyncState>("idle")
    const [accountState, setAccountState] = useState<AsyncState>("idle")
    const [decryptState, setDecryptState] = useState<AsyncState>("idle")
    const [errorMessage, setErrorMessage] = useState<string | null>(null)
    const [accountAddress, setAccountAddress] = useState<string | null>(null)
    const [userDecryptSummary, setUserDecryptSummary] = useState<readonly TypedValue[] | null>(null)
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
        makeUserDecryptable?: number
        userDecrypt?: number
    }>({})

    ////////////////////////////////////////////////////////////////////////////
    // Misc
    ////////////////////////////////////////////////////////////////////////////

    const { lines: logLines, log, clear: clearLog } = useLog()

    const isRunning =
        initState === "pending" ||
        instanceState === "pending" ||
        accountState === "pending" ||
        decryptState === "pending"
    const relayerConfig = useMemo(() => resolveFhevmChainConfig(config), [config])

    const relayerOptions: RelayerUserDecryptOptions | undefined = useMemo(() => {
        const option = {
            onProgress(args: RelayerUserDecryptProgressArgs) {
                console.log(`V2 ROUTE PROGRESS: ${args.type}`)
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
        setUserDecryptSummary(null)
        setDurations({}) // Reset durations on retry

        setInitState("pending")
        setInstanceState("idle")
        setAccountState("idle")
        setDecryptState("idle")
    }, [])

    ////////////////////////////////////////////////////////////////////////////
    // Step 1: initRuntime
    ////////////////////////////////////////////////////////////////////////////

    const initRuntime = useCallback(async () => {
        setInitState("pending")
        await nextTick()

        const start = performance.now()
        await initDecryptRuntime(relayerConfig, log)
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
        const fhevmClient = await createDecryptClient(relayerConfig)
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
    // Step 4: userDecryptValue
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
            setDurations((prev) => ({ ...prev, userDecrypt: end - start }))

            setUserDecryptSummary(null)
            setDecryptState("success")
        },
        // [relayerOptions]
        []
    )

    ////////////////////////////////////////////////////////////////////////////
    // Run Flow
    ////////////////////////////////////////////////////////////////////////////

    const runFlow = useCallback(async () => {
        resetFlowState()

        try {
            await initRuntime()
            const client = await createClient()
            const account = await loadAccount()
            const handles = getUserHandles(config, decryptType)
            await userDecryptValue(client, account, handles.handle, relayerSDKTestContractAddress)
            setCompletedAt(new Date().toISOString())
        } catch (error) {
            setInitState((prev) => (prev === "pending" ? "error" : prev))
            setInstanceState((prev) => (prev === "pending" ? "error" : prev))
            setAccountState((prev) => (prev === "pending" ? "error" : prev))
            setDecryptState((prev) => (prev === "pending" ? "error" : prev))
            setErrorMessage(`${decryptType} user decryption flow failed: ${formatError(error)}`)
        }
    }, [
        resetFlowState,
        initRuntime,
        createClient,
        loadAccount,
        userDecryptValue,
        relayerOptions,
        relayerSDKTestContractAddress,
        config,
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
                        initSDK()
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
                        createInstance()
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

                <div className="status-card" data-state={decryptState}>
                    <dt>
                        userDecrypt()
                        {durations.userDecrypt !== undefined && (
                            <span className="duration-badge">{formatDuration(durations.userDecrypt)}</span>
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

            {accountAddress || userDecryptSummary ? (
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
