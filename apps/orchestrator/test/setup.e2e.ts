import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'
import { execSync } from 'child_process'
import { randomUUID } from 'crypto'
import type { TestProject } from 'vitest/node'

let pgContainer: StartedPostgreSqlContainer | undefined = undefined
let awsContainer: StartedLocalStackContainer | undefined = undefined

async function startPostresInstance(databaseUrl: string) {
  const schema = randomUUID()
  console.log(`migrating prisma into schema ${schema}`)
  const url = `${databaseUrl}?schema=${schema}`

  // Execute Prisma migrations
  execSync('pnpx prisma migrate deploy', {
    env: { DATABASE_URL: url, PATH: process.env.PATH },
  })

  return url
}

async function startPostgres(maxWorkers: number) {
  pgContainer = await new PostgreSqlContainer('postgres:17-alpine').start()
  if (!pgContainer) {
    throw new Error('Failed to start postgres container')
  }

  const host = pgContainer.getHost()
  const port = pgContainer.getPort()
  const database = pgContainer.getDatabase()
  const username = pgContainer.getUsername()
  const password = pgContainer.getPassword()

  console.log(
    `🚛 testcontainer Postgres running on ${host}:${port}/${database}`,
  )

  const databaseUrl = `postgresql://${username}:${password}@${host}:${port}/${database}`
  const urls = await Promise.all(
    new Array(maxWorkers).fill(0).map(() => startPostresInstance(databaseUrl)),
  )

  return urls
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
  const maxWorkers = project.globalConfig.poolOptions?.forks?.maxForks ?? 10

  const [databaseUrls, awsEndpoint] = await Promise.all([
    startPostgres(maxWorkers),
    startAws(),
  ])
  project.provide('databaseUrls', databaseUrls)
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
    databaseUrls: string[]
    awsEndpoint: string
  }
}
