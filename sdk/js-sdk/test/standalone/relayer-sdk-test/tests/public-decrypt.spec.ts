import { type Page, expect, test } from "@playwright/test"

import { getPublicHandles } from "./handles"
import {
    type DecryptionTypes,
    RELAYER_CONFIG_NAMES,
    type RelayerConfigName,
    decryptionTypes,
} from "./utils"

const runPublicDecryptFlow = async (
    page: Page,
    type: DecryptionTypes,
    config: RelayerConfigName
) => {
    await page.goto(
        `http://localhost:3000/public-decrypt?config=${config}&type=${type}`
    )

    const selectedTypeChip = page.getByTestId("public-decrypt-selected-type")
    const initStatusDd = page.getByTestId("public-decrypt-init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId(
        "public-decrypt-create-instance-status-dd"
    )
    const publicDecryptionStatusDd = page.getByTestId(
        "public-decrypt-run-decryption-status-dd"
    )

    await selectedTypeChip.waitFor({ state: "visible" })
    expect(await selectedTypeChip.innerText()).toBe(`type=${type}`)

    await initStatusDd.waitFor({ state: "visible" })
    const initStatusDdText = await initStatusDd.innerText()
    expect(initStatusDdText).toBe("Ready")

    await instanceStatusDd.waitFor({ state: "visible" })
    const instanceStatusDdText = await instanceStatusDd.innerText()
    expect(instanceStatusDdText).toBe("Ready")

    await publicDecryptionStatusDd.waitFor({ state: "visible" })
    const publicDecryptionStatusDdText =
        await publicDecryptionStatusDd.innerText()
    expect(publicDecryptionStatusDdText).toBe("Ready")

    const ciphertextHandles = getPublicHandles(config, type).handle
    const decryptedHandles = page.locator(
        '[data-testid^="public-decrypt-decrypted-handle-"]'
    )
    const inputValues = getPublicHandles(config, type).clear
    const decryptedValues = page.locator(
        '[data-testid^="public-decrypt-decrypted-value-"]'
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
        test(`can publicly decrypt external e${type} on ${name}`, async ({
            page,
        }) => {
            await runPublicDecryptFlow(page, type, name)
        })
    }
}
