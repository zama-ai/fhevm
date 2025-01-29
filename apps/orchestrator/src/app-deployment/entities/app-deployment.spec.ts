import { beforeEach, describe, expect, it } from 'vitest'
import { AppDeployment } from './app-deployment.js'
import { randomUUID } from 'crypto'

import {
  completed,
  confirmSC,
  discoverSC,
  registerSC,
  requested,
  scConfirmed,
  scDiscovered,
  scRegistered,
} from 'messages'

const address = '0xa2dd817c2fdc3a2996f1a5174cf8f1aaed466e82'
const chainId = '1'

describe('AppDeployment', () => {
  let deployment: AppDeployment
  let applicationId: string
  let deploymentId: string

  beforeEach(() => {
    applicationId = randomUUID()
    deploymentId = randomUUID()
    deployment = new AppDeployment({ applicationId, deploymentId })
  })

  describe('when idle', () => {
    beforeEach(() => {
      expect(deployment.status).toBe('Idle')
    })

    describe('on deployment requested', () => {
      it('should request SC discovery', () => {
        const messages = deployment.send(
          requested({ applicationId, deploymentId, address, chainId }),
        )

        expect(messages).toEqual([
          discoverSC({ applicationId, deploymentId, address, chainId }),
        ])
      })

      it('should propagate the metadata', () => {
        const meta = { traceId: randomUUID() }
        const messages = deployment.send(
          requested({ applicationId, deploymentId, address, chainId }, meta),
        )

        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual(meta)
      })
    })
  })

  describe('when discovering', () => {
    let contractAddress: string
    let creatorAddress: string

    beforeEach(() => {
      contractAddress = randomUUID()
      creatorAddress = randomUUID()
      deployment.send(
        requested({ applicationId, deploymentId, address, chainId }),
      )
      expect(deployment.status).toBe('Discovering')
    })

    describe('on SC discovered', () => {
      it('should request SC confirmation', () => {
        const messages = deployment.send(
          scDiscovered({
            applicationId,
            deploymentId,
            contractAddress,
            creatorAddress,
          }),
        )

        expect(messages).toEqual([confirmSC({ applicationId, deploymentId })])
      })

      it('should propagate metadata', () => {
        const meta = { traceId: randomUUID() }
        const messages = deployment.send(
          scDiscovered(
            { applicationId, deploymentId, contractAddress, creatorAddress },
            meta,
          ),
        )

        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual(meta)
      })

      it('should ignore wrong identifier', () => {
        for (const key of ['applicationId', 'deploymentId']) {
          console.log(`sending scDiscover with wrong ${key}`)
          const id = randomUUID()
          const messages = deployment.send(
            scDiscovered({
              applicationId,
              deploymentId,
              contractAddress,
              creatorAddress,
              [key]: id,
            }),
          )

          expect(messages).toStrictEqual([])
        }
      })
    })
  })

  describe('when confirming', () => {
    let contractAddress: string
    let creatorAddress: string

    beforeEach(() => {
      contractAddress = randomUUID()
      creatorAddress = randomUUID()
      deployment.send(
        requested({ applicationId, deploymentId, address, chainId }),
      )
      deployment.send(
        scDiscovered({
          applicationId,
          deploymentId,
          contractAddress,
          creatorAddress,
        }),
      )
      expect(deployment.status).toBe('Confirming')
    })

    describe('on SC confirmation', () => {
      it('should request SC registration on SC confirmation', () => {
        const messages = deployment.send(
          scConfirmed({ applicationId, deploymentId }),
        )

        expect(messages).toEqual([registerSC({ applicationId, deploymentId })])
      })

      it('should request SC registration on SC confirmation', () => {
        const meta = { traceId: randomUUID() }
        const messages = deployment.send(
          scConfirmed({ applicationId, deploymentId }, meta),
        )

        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual(meta)
      })

      it('should ignore wrong identifier', () => {
        for (const key of ['applicationId', 'deploymentId']) {
          const id = randomUUID()
          const messages = deployment.send(
            scConfirmed({ applicationId, deploymentId, [key]: id }),
          )

          expect(messages).toStrictEqual([])
        }
      })
    })
  })

  describe('when registering', () => {
    let contractAddress: string
    let creatorAddress: string

    beforeEach(() => {
      contractAddress = randomUUID()
      creatorAddress = randomUUID()
      deployment.send(
        requested({ applicationId, deploymentId, address, chainId }),
      )
      deployment.send(
        scDiscovered({
          applicationId,
          deploymentId,
          contractAddress,
          creatorAddress,
        }),
      )
      deployment.send(scConfirmed({ applicationId, deploymentId }))
      expect(deployment.status).toBe('Registering')
    })

    describe('on SC registered', () => {
      it('should complete', () => {
        const messages = deployment.send(
          scRegistered({ applicationId, deploymentId }),
        )
        expect(messages).toEqual([completed({ applicationId, deploymentId })])
        expect(deployment.status).toBe('Active')
      })

      it('should complete', () => {
        const meta = { traceId: randomUUID() }
        const messages = deployment.send(
          scRegistered({ applicationId, deploymentId }, meta),
        )

        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual(meta)
      })

      it('should ignore wrong identifier', () => {
        for (const key of ['applicationId', 'deploymentId']) {
          const id = randomUUID()
          const messages = deployment.send(
            scRegistered({ applicationId, deploymentId, [key]: id }),
          )

          expect(messages).toStrictEqual([])
        }
      })
    })
  })
})
