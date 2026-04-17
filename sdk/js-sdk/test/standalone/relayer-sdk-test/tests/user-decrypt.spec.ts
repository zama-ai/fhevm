import { type Page, expect, test } from "@playwright/test"

import { getUserHandles } from "./handles"
import {
    type DecryptionTypes,
    RELAYER_CONFIG_NAMES,
    type RelayerConfigName,
    decryptionTypes,
} from "./utils"

const runUserDecryptFlow = async (
    page: Page,
    type: DecryptionTypes,
    config: RelayerConfigName
) => {
    await page.goto(
        `http://localhost:3000/user-decrypt?config=${config}&type=${type}`
    )

    const selectedTypeChip = page.getByTestId("user-decrypt-selected-type")
    const initStatusDd = page.getByTestId("user-decrypt-init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId(
        "user-decrypt-create-instance-status-dd"
    )
    const loadAccountStatusDd = page.getByTestId(
        "user-decrypt-load-account-status-dd"
    )
    const userDecryptionStatusDd = page.getByTestId(
        "user-decrypt-run-decryption-status-dd"
    )

    await selectedTypeChip.waitFor({ state: "visible" })
    expect(await selectedTypeChip.innerText()).toBe(`type=${type}`)

    await initStatusDd.waitFor({ state: "visible" })
    const initStatusDdText = await initStatusDd.innerText()
    expect(initStatusDdText).toBe("Ready")

    await instanceStatusDd.waitFor({ state: "visible" })
    const instanceStatusDdText = await instanceStatusDd.innerText()
    expect(instanceStatusDdText).toBe("Ready")

    await loadAccountStatusDd.waitFor({ state: "visible" })
    const loadAccountStatusDdText = await loadAccountStatusDd.innerText()
    expect(loadAccountStatusDdText).toBe("Ready")

    await userDecryptionStatusDd.waitFor({ state: "visible" })
    const userDecryptionStatusDdText = await userDecryptionStatusDd.innerText()
    expect(userDecryptionStatusDdText).toBe("Ready")

    const ciphertextHandles = getUserHandles(config, type).handle
    const decryptedHandles = page.locator(
        '[data-testid^="user-decrypt-decrypted-handle-"]'
    )
    const inputValues = getUserHandles(config, type).clear
    const decryptedValues = page.locator(
        '[data-testid^="user-decrypt-decrypted-value-"]'
    )

    const decryptedHandleTexts = await decryptedHandles.allTextContents()
    expect(ciphertextHandles.length).toBe(decryptedHandleTexts.length)
    for (const [index, handle] of ciphertextHandles.entries()) {
        expect(handle).toBe(decryptedHandleTexts[index])
    }

    const decryptedValueTexts = await decryptedValues.allTextContents()
    expect(inputValues.length).toBe(decryptedValueTexts.length)
    for (const [index, value] of inputValues.entries()) {
        expect(value).toBe(decryptedValueTexts[index])
    }
}

for (const type of decryptionTypes) {
    for (const name of RELAYER_CONFIG_NAMES) {
        test(`can user-decrypt external e${type} on ${name}`, async ({
            page,
        }) => {
            await runUserDecryptFlow(page, type, name)
        })
    }
}
