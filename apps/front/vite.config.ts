import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig, PluginOption } from 'vite'
import 'dotenv/config'
import react from '@vitejs/plugin-react-swc'
import tsconfigPaths from 'vite-tsconfig-paths'

const current = fileURLToPath(import.meta.url)
const root = path.dirname(current)

const getEnvVarsNames = (str: string): string[] =>
  Array.from(str.matchAll(/\${(VITE_[A-Z_]+)}/g), m => m[1])

// a custom vite plugin to inject environment variables
const RuntimeEnvPlugin: PluginOption = {
  name: 'runtime-env-plugin',
  configureServer(server) {
    server.middlewares.use('/scripts/env.js', (_, res) => {
      const configContent = fs.readFileSync(
        path.resolve(root, './src/public/scripts/env.template.js'),
        'utf-8',
      )
      let content = configContent
      getEnvVarsNames(configContent).forEach(v => {
        if (process.env?.[v] && process.env?.[v]?.length) {
          content = content.replace('${' + v + '}', process.env[v])
        } else {
          content = content.replace('${' + v + '}', '')
        }
      })
      res.setHeader('content-type', 'application/javascript')
      res.end(content)
    })
  },
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [RuntimeEnvPlugin, react(), tsconfigPaths()],
})
