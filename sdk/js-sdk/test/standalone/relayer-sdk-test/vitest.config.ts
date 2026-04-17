import react from "@vitejs/plugin-react"
import { playwright } from "@vitest/browser-playwright"
import { defineConfig } from "vitest/config"

export default defineConfig({
    plugins: [react()],
    test: {
        include: ["src/**/*.test.{js,ts,jsx,tsx}"],
        browser: {
            enabled: true,
            provider: playwright(),
            instances: [{ browser: "chromium" }],
        },
    },
})
