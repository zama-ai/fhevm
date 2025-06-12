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
  console.log(`\x1b[32mmigrating prisma into schema ${schema}\x1b[0m`)
  const url = `${databaseUrl}?schema=${schema}`

  try {
    // Execute Prisma migrations
    execSync('pnpx prisma migrate deploy', {
      env: { APP__DB__URL: url, PATH: process.env.PATH },
    })
  } catch (error) {
    console.error(`\x1b[33m🛑 Failed to migrate prisma: ${error} \x1b[0m`)
    throw error
  }

  return url
}

async function startPostgres(maxWorkers: number) {
  try {
    console.log(`\x1b[32mstarting postgres:17-alpine container\x1b[0m`)
    pgContainer = await new PostgreSqlContainer('postgres:17-alpine').start()
    if (!pgContainer) {
      throw new Error('\x1b[33m⛔️ Failed to start postgres container\x1b[0m')
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
      new Array(maxWorkers)
        .fill(0)
        .map(() => startPostresInstance(databaseUrl)),
    )

    try {
      // NOTE: I need to run the prisma client to generate the prisma client
      // Sometimes, it doesn't get the last generated client.
      // I need to run it just once
      console.log(`\x1b[32mrunning pnpx prisma generate\x1b[0m`)
      execSync('pnpx prisma generate', {
        env: { APP__DB__URL: urls[0], PATH: process.env.PATH },
      })
    } catch (error) {
      console.error(
        `\x1b[33m🛑 Failed to generate prisma client: ${error} \x1b[0m`,
      )
      throw error
    }

    return urls
  } catch (error) {
    console.error(`\x1b[33m🛑 Failed to start postgres: ${error} \x1b[0m`)
    throw error
  }
}

async function stopPostgres() {
  if (pgContainer) {
    console.log(`\x1b[33m🛑 testcontainer stopping Postgres\x1b[0m`)
    try {
      await pgContainer.stop()
    } catch (error) {
      console.warn(`\x1b[33m🛑 Failed to stop postgres: ${error} \x1b[0m`)
    }
  }
}

async function startAws() {
  try {
    console.log(
      `\x1b[32mstarting localstack/localstack:latest container\x1b[0m`,
    )
    awsContainer = await new LocalstackContainer(
      'localstack/localstack:latest',
    ).start()

    const connectionUri = awsContainer.getConnectionUri()
    console.log(
      `\x1b[32m🚛 testcontainer AWS running on ${connectionUri}\x1b[0m`,
    )

    return connectionUri
  } catch (error) {
    console.error(`\x1b[33m🛑 Failed to start localstack: ${error} \x1b[0m`)
    throw error
  }
}

async function stopAws() {
  if (awsContainer) {
    console.log(`\x1b[33m🛑 testcontainer stopping Postgres\x1b[0m`)
    try {
      await awsContainer.stop()
    } catch (error) {
      console.warn(`\x1b[33m🛑 Failed to stop localstack: ${error} \x1b[0m`)
    }
  }
}

async function startRedis() {
  try {
    console.log(`\x1b[32mstarting redis:7.4-alpine container\x1b[0m`)
    redisContainer = await new RedisContainer('redis:7.4-alpine').start()

    if (!redisContainer) {
      throw new Error('Failed to start postgres container')
    }

    const host = redisContainer.getHost()
    const port = redisContainer.getPort()

    console.log(
      `\x1b[32m🚛 testcontainer Redis running on ${host}:${port}\x1b[0m`,
    )

    return { host, port }
  } catch (error) {
    console.error(`\x1b[33m🛑 Failed to start redis: ${error} \x1b[0m`)
    throw error
  }
}

async function stopRedis() {
  if (redisContainer) {
    console.log(`\x1b[33m🛑 testcontainer stopping Redis\x1b[0m`)
    try {
      await redisContainer.stop()
    } catch (error) {
      console.warn(`\x1b[33m🛑 Failed to stop redis: ${error} \x1b[0m`)
    }
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
