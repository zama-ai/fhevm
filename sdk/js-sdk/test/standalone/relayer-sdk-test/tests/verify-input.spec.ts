import { type Page, expect, test } from "@playwright/test"

import { RELAYER_CONFIG_NAMES, type RelayerConfigName } from "./utils"

const runVerifyFlow = async (page: Page, config: RelayerConfigName) => {
    await page.goto(`http://localhost:3000/verify-input?config=${config}`)

    const initStatusDd = page.getByTestId("verify-input-init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId(
        "verify-input-create-instance-status-dd"
    )
    const loadAccountStatusDd = page.getByTestId(
        "verify-input-load-account-status-dd"
    )
    const createInputStatusDd = page.getByTestId(
        "verify-input-create-input-status-dd"
    )
    const populateBufferStatusDd = page.getByTestId(
        "verify-input-populate-buffer-status-dd"
    )
    const zkProofStatusDd = page.getByTestId(
        "verify-input-run-generate-zk-proof-status-dd"
    )
    const verifZkpStatusDd = page.getByTestId(
        "verify-input-run-verif-zkp-status-dd"
    )
    const verifyInputStatusDd = page.getByTestId(
        "verify-input-run-verify-input-status-dd"
    )

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

    await verifyInputStatusDd.waitFor({ state: "visible" })
    const verifyInputStatusDdText = await verifyInputStatusDd.innerText()
    expect(verifyInputStatusDdText).toBe("Ready")

    const ciphertextHandles = page.locator(
        '[data-testid^="verify-input-ciphertext-handle-"]'
    )
    const verifiedCiphertextHandles = page.locator(
        '[data-testid^="verify-input-verified-ciphertext-handle-"]'
    )
    const ciphertextHandleTexts = await ciphertextHandles.allTextContents()
    const verifiedCiphertextHandleTexts =
        await verifiedCiphertextHandles.allTextContents()

    expect(ciphertextHandleTexts.length).toBe(
        verifiedCiphertextHandleTexts.length
    )
    for (const [index, handle] of ciphertextHandleTexts.entries()) {
        expect(handle).toBe(verifiedCiphertextHandleTexts[index])
    }
}

for (const name of RELAYER_CONFIG_NAMES) {
    test(`can verify the input proof against the encrypted values on ${name}`, async ({
        page,
    }) => {
        await runVerifyFlow(page, name)
    })
}
