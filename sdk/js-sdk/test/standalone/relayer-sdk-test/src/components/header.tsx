import { Link } from "@tanstack/react-router"
import { useState } from "react"

import type { FhevmChainName } from "../utils/const"
import "./header.css"

function Header() {
    // State for the toggles
    const [isV2, setIsV2] = useState(false) // false = V1, true = V2
    const [isTestnet, setIsTestnet] = useState(true) // false = Devnet, true = Testnet

    // Logic:
    // V1 + Devnet = "devnet"
    // V1 + Testnet = "testnet"
    // V2 + Devnet = "devnetV2"
    // V2 + Testnet = "testnetV2"
    const configValue: FhevmChainName = `${isTestnet ? "testnet" : "devnet"}${isV2 ? "V2" : ""}`

    return (
        <div className="header-shell">
            <div className="header-left">
                <h1 className="header-title">
                    @zama-ai/relayer-sdk Test Harness
                </h1>

                {/* Controls Area */}
                <div className="header-controls">
                    {/* Network Toggle */}
                    <label className="toggle-label">
                        <span className={!isTestnet ? "active" : ""}>
                            Devnet
                        </span>
                        <div className="toggle-switch">
                            <input
                                type="checkbox"
                                checked={isTestnet}
                                onChange={(e) => {
                                    setIsTestnet(e.target.checked)
                                }}
                            />
                            <span className="slider"></span>
                        </div>
                        <span className={isTestnet ? "active" : ""}>
                            Testnet
                        </span>
                    </label>

                    {/* Version Toggle */}
                    <label className="toggle-label">
                        <span className={!isV2 ? "active" : ""}>V1</span>
                        <div className="toggle-switch">
                            <input
                                type="checkbox"
                                checked={isV2}
                                onChange={(e) => {
                                    setIsV2(e.target.checked)
                                }}
                            />
                            <span className="slider"></span>
                        </div>
                        <span className={isV2 ? "active" : ""}>V2</span>
                    </label>
                </div>
            </div>

            <div className="header-routes">
                <Link className="route-label" key="home" to="/">
                    Home
                </Link>
                <Link
                    className="route-label"
                    key="initialization"
                    to="/init"
                    search={{ config: configValue }}
                >
                    Initialization
                </Link>
                <Link
                    className="route-label"
                    key="encrypt"
                    to="/encrypt"
                    search={{ config: configValue }}
                >
                    Encryption
                </Link>
                <Link
                    className="route-label"
                    key="zkproof"
                    to="/zkproof"
                    search={{ config: configValue }}
                >
                    ZK Proof
                </Link>
                <Link
                    className="route-label"
                    key="verify-input"
                    to="/verify-input"
                    search={{ config: configValue }}
                >
                    Input verification
                </Link>
                <Link
                    className="route-label"
                    key="public-decryption"
                    to="/public-decrypt"
                    search={{ config: configValue, type: "bool" }}
                >
                    Public decryption
                </Link>
                <Link
                    className="route-label"
                    key="user-decryption"
                    to="/user-decrypt"
                    search={{ config: configValue, type: "bool" }}
                >
                    User decryption
                </Link>
                <Link
                    className="route-label"
                    key="user-decryption-multi"
                    to="/user-decrypt-multi"
                    search={{ config: configValue, type: "bool" }}
                >
                    User decryption multi contracts
                </Link>
                <Link
                    className="route-label"
                    key="public-decryption-fresh-handles"
                    to="/public-decrypt-fresh-handles"
                    search={{ config: configValue, type: "bool" }}
                >
                    Public decryption (Fresh Handles)
                </Link>
                <Link
                    className="route-label"
                    key="user-decryption-fresh-handles"
                    to="/user-decrypt-fresh-handles"
                    search={{ config: configValue, type: "bool" }}
                >
                    User decryption (Fresh Handles)
                </Link>

                <Link
                    className="route-label"
                    key="user-decryption-multi-fresh-handles"
                    to="/user-decrypt-multi-fresh-handles"
                    search={{ config: configValue, type: "bool" }}
                >
                    User decryption multi contracts (Fresh Handles)
                </Link>
            </div>
        </div>
    )
}

export default Header
