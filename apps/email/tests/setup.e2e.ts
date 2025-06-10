import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'

import type { TestProject } from 'vitest/node'

declare module 'vitest' {
  export interface ProvidedContext {
    maxWorkers: number
    awsEndpoint: string
  }
}

export async function setup(project: TestProject) {
  const maxWorkers = project.globalConfig.poolOptions?.forks?.maxForks ?? 10
  const [connectionUri] = await Promise.all([startAws()])
  project.provide('maxWorkers', maxWorkers)
  project.provide('awsEndpoint', connectionUri)
}

export async function teardown() {
  await Promise.all([stopAws()])
}

let awsContainer: StartedLocalStackContainer | undefined = undefined
async function startAws(): Promise<string> {
  awsContainer = await new LocalstackContainer('localstack/localstack:latest')
    .withEnvironment({
      SERVICES: 'sqs,ses,s3',
    })
    .start()

  const connectionUri = awsContainer.getConnectionUri()
  console.log(
    `\x1b[40m\x1b[32m🚛 testcontainer AWS running on ${connectionUri}\x1b[0m`,
  )

  return connectionUri
}

async function stopAws() {}
