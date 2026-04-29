import { type Page, expect, test } from "@playwright/test"

import { RELAYER_CONFIG_NAMES, type RelayerConfigName } from "./utils"

const runZkProofFlow = async (page: Page, config: RelayerConfigName) => {
    await page.goto(`http://localhost:3000/zkproof?config=${config}`)

    // Get the locator of the element showing if the function succeeded or failed.
    // It allows to wait for each step to return a value (success/failure).
    const initStatusDd = page.getByTestId("zk-proof-init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId(
        "zk-proof-create-instance-status-dd"
    )
    const createInputStatusDd = page.getByTestId(
        "zk-proof-create-input-status-dd"
    )
    const populateBufferStatusDd = page.getByTestId(
        "zk-proof-populate-buffer-status-dd"
    )
    const zkProofStatusDd = page.getByTestId(
        "zk-proof-run-generate-zk-proof-status-dd"
    )

    // Wait for the success/failure of the workflow step.
    // Then, assert that it succeeded.
    await initStatusDd.waitFor({ state: "visible" })
    const initStatusDdText = await initStatusDd.innerText()
    expect(initStatusDdText).toBe("Ready")

    await instanceStatusDd.waitFor({ state: "visible" })
    const instanceStatusDdText = await instanceStatusDd.innerText()
    expect(instanceStatusDdText).toBe("Ready")

    await createInputStatusDd.waitFor({ state: "visible" })
    const createInputStatusDdText = await createInputStatusDd.innerText()
    expect(createInputStatusDdText).toBe("Ready")

    await populateBufferStatusDd.waitFor({ state: "visible" })
    const populateBufferStatusDdText = await populateBufferStatusDd.innerText()
    expect(populateBufferStatusDdText).toBe("Ready")

    await zkProofStatusDd.waitFor({ state: "visible" })
    const zkProofStatusDdText = await zkProofStatusDd.innerText()
    expect(zkProofStatusDdText).toBe("Ready")
}

for (const name of RELAYER_CONFIG_NAMES) {
    test(`can generate ZK Proof on ${name} config`, async ({ page }) => {
        await runZkProofFlow(page, name)
    })
}
