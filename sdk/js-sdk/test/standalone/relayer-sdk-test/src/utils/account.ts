import type { Hex } from "viem"
import { mnemonicToAccount, privateKeyToAccount } from "viem/accounts"

export interface AccountKey {
    mnemonic?: string
    privateKey?: Hex
}

/**
 *
 * @param key - The object containing either the mnemonic or private key of the account to be loaded.
 * @returns The loaded account from the provided credentials.
 * @notice The mnemonic prevails over the private key.
 */
export const loadWalletAccount = (key: AccountKey) => {
    if (key.mnemonic) {
        return mnemonicToAccount(key.mnemonic)
    } else if (key.privateKey) {
        return privateKeyToAccount(key.privateKey)
    }
    throw new Error("A mnemonic or a private key must be provided.")
}
