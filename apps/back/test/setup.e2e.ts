import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'
import { RedisContainer, StartedRedisContainer } from '@testcontainers/redis'

import type { TestProject } from 'vitest/node'

let pgContainer: StartedPostgreSqlContainer | undefined = undefined
let awsContainer: StartedLocalStackContainer | undefined = undefined
let redisContainer: StartedRedisContainer | undefined = undefined

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

  console.log(
    `🚛 testcontainer Postgres running on ${host}:${port}/${database}`,
  )

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
  const [databaseUrl, awsEndpoint, redisConnection] = await Promise.all([
    startPostgres(),
    startAws(),
    startRedis(),
  ])
  project.provide('databaseUrl', databaseUrl)
  project.provide('awsEndpoint', awsEndpoint)
  project.provide('redisConnection', redisConnection)
}

export async function teardown() {
  await Promise.all([stopPostgres(), stopAws(), stopRedis()])
}

declare module 'vitest' {
  export interface ProvidedContext {
    databaseUrl: string
    awsEndpoint: string
    redisConnection: { host: string; port: number }
  }
}
