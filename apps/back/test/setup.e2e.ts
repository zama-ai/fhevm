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
  console.log(`🚛 testcontainer Redis running on ${host}:${port}`)

  return { host, port }
}

async function stopRedis() {
  if (redisContainer) {
    await redisContainer.stop()
  }
}

export async function setup(project: TestProject) {
  const maxWorkers = project.globalConfig.poolOptions?.forks?.maxForks ?? 10
  const [databaseUrls, awsEndpoint, redisConnection] = await Promise.all([
    startPostgres(maxWorkers),
    startAws(),
    startRedis(),
  ])
  project.provide('maxWorkers', maxWorkers)
  project.provide('databaseUrls', databaseUrls)
  project.provide('awsEndpoint', awsEndpoint)
  project.provide('redisConnection', redisConnection)
}

export async function teardown() {
  await Promise.all([stopPostgres(), stopAws(), stopRedis()])
}

declare module 'vitest' {
  export interface ProvidedContext {
    maxWorkers: number
    databaseUrls: string[]
    awsEndpoint: string
    redisConnection: { host: string; port: number }
  }
}
