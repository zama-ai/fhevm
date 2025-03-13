import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'
import { RedisContainer, StartedRedisContainer } from '@testcontainers/redis'
import { execSync } from 'child_process'
import { randomUUID } from 'crypto'
import type { TestProject } from 'vitest/node'

let pgContainer: StartedPostgreSqlContainer | undefined = undefined
let awsContainer: StartedLocalStackContainer | undefined = undefined
let redisContainer: StartedRedisContainer | undefined = undefined

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
    `\x1b[32m🚛 testcontainer Postgres running on ${host}:${port}/${database}\x1b[0m`,
  )

  const databaseUrl = `postgresql://${username}:${password}@${host}:${port}/${database}`
  const urls = await Promise.all(
    new Array(maxWorkers).fill(0).map(() => startPostresInstance(databaseUrl)),
  )

  return urls
}

async function stopPostgres() {
  if (pgContainer) {
    console.log(`\x1b[33m🛑 testcontainer stopping Postgres\x1b[0m`)
    await pgContainer.stop()
  }
}

async function startAws() {
  awsContainer = await new LocalstackContainer(
    'localstack/localstack:latest',
  ).start()

  const connectionUri = awsContainer.getConnectionUri()
  console.log(`\x1b[32m🚛 testcontainer AWS running on ${connectionUri}\x1b[0m`)

  return connectionUri
}

async function stopAws() {
  if (awsContainer) {
    console.log(`\x1b[33m🛑 testcontainer stopping Postgres\x1b[0m`)
    await awsContainer.stop()
  }
}

async function startRedis() {
  redisContainer = await new RedisContainer('redis:7.4-alpine').start()

  if (!redisContainer) {
    throw new Error('Failed to start postgres container')
  }

  const host = redisContainer.getHost()
  const port = redisContainer.getPort()

  // TODO REMOVE THIS
  console.log(
    `\x1b[32m🚛 testcontainer Redis running on ${host}:${port}\x1b[0m`,
  )

  return { host, port }
}

async function stopRedis() {
  if (redisContainer) {
    await redisContainer.stop()
    console.log(`\x1b[33m🛑 testcontainer Redis stopped\x1b[0m`)
  }
}

export default async function setup(project: TestProject) {
  const maxWorkers = project.globalConfig.poolOptions?.forks?.maxForks ?? 10
  project.provide('maxWorkers', maxWorkers)

  const [databaseUrls, awsEndpoint, redisConnection] = await Promise.all([
    startPostgres(maxWorkers),
    startAws(),
    startRedis(),
  ])
  project.provide('databaseUrls', databaseUrls)
  project.provide('awsEndpoint', awsEndpoint)
  project.provide('redisConnection', redisConnection)

  project.onTestsRerun(() => {
    console.log(`global setup: rerunning...`)
  })

  return async () => {
    await Promise.all([stopPostgres(), stopAws(), stopRedis()])
  }
}

declare module 'vitest' {
  export interface ProvidedContext {
    maxWorkers: number
    databaseUrls: string[]
    awsEndpoint: string
    redisConnection: { host: string; port: number }
  }
}
