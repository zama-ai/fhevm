import { CodegenConfig } from '@graphql-codegen/cli'
import { config } from 'dotenv'
config()

const genConfig: CodegenConfig = {
  schema: '../back/src/infra/graphql/schema.gql', // process.env.VITE_BACK_HTTP_URL,
  documents: [
    'src/**/*.tsx',
    'src/queries.ts',
    'src/**/use*.ts',
    'src/pages/*.loader.ts',
  ],
  generates: {
    './src/__generated__/': {
      preset: 'client',
    },
  },
}

export default genConfig
