import { CodegenConfig } from '@graphql-codegen/cli'
import { config } from 'dotenv'
config()

const genConfig: CodegenConfig = {
  schema: process.env.VITE_BACK_HTTP_URL,
  documents: [
    'src/**/*.tsx',
    'src/queries.ts',
    'src/**/*.loader.ts',
    'src/hooks/*.ts',
  ],
  generates: {
    './src/__generated__/': {
      preset: 'client',
    },
  },
}

export default genConfig
