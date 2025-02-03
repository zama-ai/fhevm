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
  pgContainer = await new PostgreSqlContainer('postgres:17-alpine').start()
  if (!pgContainer) {
    throw new Error('Failed to start postgres container')
  }
  return pgContainer.getConnectionUri()
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
  const [databaseUrl, awsEndpoint] = await Promise.all([
    startPostgres(),
    startAws(),
  ])
  project.provide('databaseUrl', databaseUrl)
  project.provide('awsEndpoint', awsEndpoint)
}

export async function teardown() {
  await Promise.all([stopPostgres(), stopAws()])
}

declare module 'vitest' {
  export interface ProvidedContext {
    databaseUrl: string
    awsEndpoint: string
  }
}
