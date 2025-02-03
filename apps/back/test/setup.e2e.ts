import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'
import type { TestProject } from 'vitest/node'

let pgContainer: StartedPostgreSqlContainer | undefined = undefined
let awsContainer: StartedLocalStackContainer | undefined = undefined

async function startPostgres() {
  // Note: for better integration tests, keep the database image aligned with the one used in production
  pgContainer = await new PostgreSqlContainer('postgres:17-alpine').start()

  if (!pgContainer) {
    throw new Error('Failed to start postgres container')
  }

  const host = pgContainer.getHost()
  const port = pgContainer.getPort()
  const database = pgContainer.getDatabase()
  const username = pgContainer.getUsername()
  const password = pgContainer.getPassword()

  return `postgresql://${username}:${password}@${host}:${port}/${database}`
}

async function stopPostgres() {
  if (pgContainer) {
    await pgContainer.stop()
  }
}

async function startAws() {
  awsContainer = await new LocalstackContainer(
    'localstack/localstack:latest',
  ).start()

  return awsContainer.getConnectionUri()
}

async function stopAws() {
  if (awsContainer) {
    await awsContainer.stop()
  }
}

export async function setup(project: TestProject) {
  const [databaseUrl, connectionUri] = await Promise.all([
    startPostgres(),
    startAws(),
  ])
  project.provide('databaseUrl', databaseUrl)
  project.provide('connectionUri', connectionUri)
}

export async function teardown() {
  await Promise.all([stopPostgres(), stopAws()])
}

declare module 'vitest' {
  export interface ProvidedContext {
    databaseUrl: string
    connectionUri: string
  }
}
