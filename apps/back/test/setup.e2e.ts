import { execSync } from 'child_process'
import { randomUUID } from 'crypto'

import type { TestProject } from 'vitest/node'

async function startPostresInstance(databaseUrl: string) {
  const schema = randomUUID()
  console.log(`migrating prisma into schema ${schema}`)
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

export async function setup(project: TestProject) {
  const maxWorkers = project.globalConfig.poolOptions?.forks?.maxForks ?? 10
  project.provide('maxWorkers', maxWorkers)
}

export async function teardown() {}

declare module 'vitest' {
  export interface ProvidedContext {
    maxWorkers: number
  }
}
