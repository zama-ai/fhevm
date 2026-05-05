import { type Page, expect, test } from "@playwright/test"

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
    page.on("console", (msg) => {
        console.log(`[browser] ${msg.type()}: ${msg.text()}`)
    })

    await page.goto(
        `http://localhost:3000/user-decrypt-fresh-handles?config=${config}&type=${type}`
    )

    const selectedTypeChip = page.getByTestId("user-decrypt-selected-type")
    const initStatusDd = page.getByTestId("user-decrypt-init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId(
        "user-decrypt-create-instance-status-dd"
    )
    const loadAccountStatusDd = page.getByTestId(
        "user-decrypt-load-account-status-dd"
    )
    const createInputStatusDd = page.getByTestId(
        "user-decrypt-create-input-status-dd"
    )
    const populateBufferStatusDd = page.getByTestId(
        "user-decrypt-populate-buffer-status-dd"
    )
    const zkProofStatusDd = page.getByTestId(
        "user-decrypt-run-generate-zk-proof-status-dd"
    )
    const verifZkpStatusDd = page.getByTestId(
        "user-decrypt-run-verif-zkp-status-dd"
    )
    const makeUserDecryptableStatusDd = page.getByTestId(
        "user-decrypt-run-make-user-decryptable-status-dd"
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

    await createInputStatusDd.waitFor({ state: "visible" })
    const createInputStatusDdText = await createInputStatusDd.innerText()
    expect(createInputStatusDdText).toBe("Ready")

    await populateBufferStatusDd.waitFor({ state: "visible" })
    const populateBufferStatusDdText = await populateBufferStatusDd.innerText()
    expect(populateBufferStatusDdText).toBe("Ready")

    await zkProofStatusDd.waitFor({ state: "visible" })
    const zkProofStatusDdText = await zkProofStatusDd.innerText()
    expect(zkProofStatusDdText).toBe("Ready")

    await verifZkpStatusDd.waitFor({ state: "visible" })
    const verifZkpStatusDdText = await verifZkpStatusDd.innerText()
    expect(verifZkpStatusDdText).toBe("Ready")

    await makeUserDecryptableStatusDd.waitFor({ state: "visible" })
    const makeUserDecryptableStatusDdText =
        await makeUserDecryptableStatusDd.innerText()
    expect(makeUserDecryptableStatusDdText).toBe("Ready")

    await userDecryptionStatusDd.waitFor({ state: "visible" })
    const userDecryptionStatusDdText = await userDecryptionStatusDd.innerText()
    expect(userDecryptionStatusDdText).toBe("Ready")

    const ciphertextHandles = page.locator(
        '[data-testid^="user-decrypt-ciphertext-handles-"]'
    )
    const decryptedHandles = page.locator(
        '[data-testid^="user-decrypt-decrypted-handle-"]'
    )
    const inputValues = page.locator(
        '[data-testid^="user-decrypt-input-values-"]'
    )
    const decryptedValues = page.locator(
        '[data-testid^="user-decrypt-decrypted-value-"]'
    )

    const ciphertextHandleTexts = await ciphertextHandles.allTextContents()
    const decryptedHandleTexts = await decryptedHandles.allTextContents()
    expect(ciphertextHandleTexts.length).toBe(decryptedHandleTexts.length)
    for (const [index, handle] of ciphertextHandleTexts.entries()) {
        expect(handle).toBe(decryptedHandleTexts[index])
    }

    const inputValueTexts = await inputValues.allTextContents()
    const decryptedValueTexts = await decryptedValues.allTextContents()
    expect(inputValueTexts.length).toBe(decryptedValueTexts.length)
    for (const [index, value] of inputValueTexts.entries()) {
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
