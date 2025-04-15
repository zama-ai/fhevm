import { registerAs } from '@nestjs/config'

export default registerAs('common', () => ({
  port: process.env.PORT ?? 3000,
  nodeEnv: process.env.NODE_ENV ?? 'development',
  logLevel:
    process.env.NODE_ENV === 'test'
      ? 'silent'
      : (process.env.LOG_LEVEL ?? 'info'),
  graphqlMaxComplexity: process.env.GRAPHQL_MAX_COMPLEXITY ?? '150',
}))
