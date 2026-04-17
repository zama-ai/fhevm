import { type Page, expect, test } from "@playwright/test"

import { RELAYER_CONFIG_NAMES, type RelayerConfigName } from "./utils"

const runInitFlow = async (page: Page, config: RelayerConfigName) => {
    await page.goto(`http://localhost:3000/init?config=${config}`)

    const initStatusDd = page.getByTestId("init-sdk-status-dd")
    const instanceStatusDd = page.getByTestId("create-instance-status-dd")

    await initStatusDd.waitFor({ state: "visible" })
    const initStatusDdText = await initStatusDd.innerText()
    expect(initStatusDdText).toBe("Ready")

    await instanceStatusDd.waitFor({ state: "visible" })
    const instanceStatusDdText = await instanceStatusDd.innerText()
    expect(instanceStatusDdText).toBe("Ready")
}

for (const name of RELAYER_CONFIG_NAMES) {
    test(`can create relayer-sdk instance on ${name} config`, async ({
        page,
    }) => {
        await runInitFlow(page, name)
    })
}
