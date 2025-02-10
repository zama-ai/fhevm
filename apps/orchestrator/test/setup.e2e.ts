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

  const host = pgContainer.getHost()
  const port = pgContainer.getPort()
  const database = pgContainer.getDatabase()

  console.log(
    `🚛 testcontainer Postgres running on ${host}:${port}/${database}`,
  )

  return pgContainer.getConnectionUri()
}

async function stopPostgres() {
  if (pgContainer) {
    console.log(`🛑 testcontainer stopping Postgres`)
    await pgContainer.stop()
  }
}

async function startAws() {
  awsContainer = await new LocalstackContainer(
    'localstack/localstack:latest',
  ).start()

  const connectionUri = awsContainer.getConnectionUri()
  console.log(`🚛 testcontainer AWS running on ${connectionUri}`)

  return connectionUri
}

async function stopAws() {
  if (awsContainer) {
    console.log(`🛑 testcontainer stopping Postgres`)
    await awsContainer.stop()
  }
}

export default async function setup(project: TestProject) {
  const [databaseUrl, awsEndpoint] = await Promise.all([
    startPostgres(),
    startAws(),
  ])
  project.provide('databaseUrl', databaseUrl)
  project.provide('awsEndpoint', awsEndpoint)

  project.onTestsRerun(() => {
    console.log(`global setup: rerunning...`)
  })

  return async () => {
    await Promise.all([stopPostgres(), stopAws()])
  }
}

declare module 'vitest' {
  export interface ProvidedContext {
    databaseUrl: string
    awsEndpoint: string
  }
}
