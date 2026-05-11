import { type Page, expect, test } from "@playwright/test"

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
    page.on("console", (msg) => {
        console.log(`[browser] ${msg.type()}: ${msg.text()}`)
    })

    await page.goto(
        `http://localhost:3000/user-decrypt-multi-fresh-handles?config=${config}&type=${type}`
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
    const createInputStatusDd = page.getByTestId(
        "user-decrypt-multi-create-input-status-dd"
    )
    const populateBufferStatusDd = page.getByTestId(
        "user-decrypt-multi-populate-buffer-status-dd"
    )
    const zkProofStatusDd = page.getByTestId(
        "user-decrypt-multi-run-generate-zk-proof-status-dd"
    )
    const verifZkpStatusDd = page.getByTestId(
        "user-decrypt-multi-run-verif-zkp-status-dd"
    )
    const makeUserDecryptableStatusDd = page.getByTestId(
        "user-decrypt-multi-run-make-user-decryptable-status-dd"
    )
    const makeUserDecryptableMirrorStatusDd = page.getByTestId(
        "user-decrypt-multi-run-make-user-decryptable-status-dd"
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

    await makeUserDecryptableMirrorStatusDd.waitFor({ state: "visible" })
    const makeUserDecryptableMirrorStatusDdText =
        await makeUserDecryptableMirrorStatusDd.innerText()
    expect(makeUserDecryptableMirrorStatusDdText).toBe("Ready")

    await userDecryptionStatusDd.waitFor({ state: "visible" })
    const userDecryptionStatusDdText = await userDecryptionStatusDd.innerText()
    expect(userDecryptionStatusDdText).toBe("Ready")

    const ciphertextHandles = await page
        .locator('[data-testid^="user-decrypt-multi-ciphertext-handle-"]')
        .allTextContents()
    const ciphertextMirrorHandles = await page
        .locator(
            '[data-testid^="user-decrypt-multi-ciphertext-mirror-handle-"]'
        )
        .allTextContents()
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

    const inputValues = await page
        .locator('[data-testid^="user-decrypt-multi-input-value-"]')
        .allTextContents()
    const inputMirrorValues = await page
        .locator('[data-testid^="user-decrypt-multi-input-mirror-value-"]')
        .allTextContents()
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
