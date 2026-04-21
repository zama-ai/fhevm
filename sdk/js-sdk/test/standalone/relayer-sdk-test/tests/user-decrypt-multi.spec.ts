import { type Page, expect, test } from "@playwright/test"

import { getUserHandles } from "./handles"
import {
    type DecryptionTypes,
    RELAYER_CONFIG_NAMES,
    type RelayerConfigName,
    decryptionTypes,
} from "./utils"

const runUserDecryptMultiFlow = async (
    page: Page,
    type: DecryptionTypes,
    config: RelayerConfigName
) => {
    await page.goto(
        `http://localhost:3000/user-decrypt-multi?config=${config}&type=${type}`
    )

    const selectedTypeChip = page.getByTestId(
        "user-decrypt-multi-selected-type"
    )
    const initStatusDd = page.getByTestId(
        "user-decrypt-multi-init-sdk-status-dd"
    )
    const instanceStatusDd = page.getByTestId(
        "user-decrypt-multi-create-instance-status-dd"
    )
    const loadAccountStatusDd = page.getByTestId(
        "user-decrypt-multi-load-account-status-dd"
    )
    const userDecryptionStatusDd = page.getByTestId(
        "user-decrypt-multi-run-decryption-status-dd"
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
    const ciphertextMirrorHandles = getUserHandles(config, type, true).handle
    const decryptedHandles = await page
        .locator(
            '[data-testid^="user-decrypt-multi-decrypted-ciphertext-handle-"]'
        )
        .allTextContents()

    const concatenatedHandles = [
        ...ciphertextHandles,
        ...ciphertextMirrorHandles,
    ]
    expect(concatenatedHandles.length).toBe(decryptedHandles.length)
    for (const [index, handle] of concatenatedHandles.entries()) {
        expect(handle).toBe(decryptedHandles[index])
    }

    const inputValues = getUserHandles(config, type).clear
    const inputMirrorValues = getUserHandles(config, type, true).clear
    const decryptedValues = await page
        .locator('[data-testid^="user-decrypt-multi-decrypted-value-"]')
        .allTextContents()

    const concatenatedValues = [...inputValues, ...inputMirrorValues]
    expect(concatenatedValues.length).toBe(decryptedValues.length)
    for (const [index, value] of concatenatedValues.entries()) {
        expect(value).toBe(decryptedValues[index])
    }
}

for (const type of decryptionTypes) {
    for (const name of RELAYER_CONFIG_NAMES) {
        test(`can user-decrypt-multi external e${type} on ${name}`, async ({
            page,
        }) => {
            await runUserDecryptMultiFlow(page, type, name)
        })
    }
}
