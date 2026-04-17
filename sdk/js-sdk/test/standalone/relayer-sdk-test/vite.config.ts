import { tanstackRouter } from "@tanstack/router-plugin/vite"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

// https://vite.dev/config/
export default defineConfig({
    build: {
        sourcemap: true,
    },
    plugins: [
        tanstackRouter({
            target: "react",
            // Allows you to optimize your application's bundle size by lazily loading route components and their associated data.
            // See https://tanstack.com/router/v1/docs/framework/react/guide/automatic-code-splitting for more details
            autoCodeSplitting: true,
            quoteStyle: "double",
        }),
        react(),
        // Adds the COEP/COOP headers to the dev server.
        // They are required for using SharedArrayBuffer on the browser.
        {
            name: "isolation",
            configureServer(server) {
                server.middlewares.use((_req, res, next) => {
                    res.setHeader("Cross-Origin-Opener-Policy", "same-origin")
                    res.setHeader(
                        "Cross-Origin-Embedder-Policy",
                        "require-corp"
                    )
                    next()
                })
            },
        },
    ],
    // Required by Vite using browserstack in local mode.
    server: {
        host: "0.0.0.0",
        allowedHosts: ["bs-local.com"],
    },
})
