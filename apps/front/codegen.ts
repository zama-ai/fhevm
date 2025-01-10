import { CodegenConfig } from '@graphql-codegen/cli'
import { config } from 'dotenv'
config()

const genConfig: CodegenConfig = {
  schema: process.env.VITE_API_URL,
  documents: ['src/**/*.tsx', 'src/queries.ts', 'src/**/*.loader.ts'],
  generates: {
    './src/__generated__/': {
      preset: 'client',
    },
  },
}

export default genConfig
