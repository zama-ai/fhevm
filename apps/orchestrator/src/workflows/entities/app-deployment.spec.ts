import { beforeEach, describe, expect, it } from 'vitest'
import { AppDeployment, AppDeploymentEvents } from './app-deployment.js'
import { faker } from '@faker-js/faker'

import { back, web3 } from 'messages'

describe('AppDeployment', () => {
  let deployment: AppDeployment
  let requestId: string
  let dAppId: string
  let chainId: string
  let address: string
  let correlationId: string

  beforeEach(() => {
    requestId = faker.string.uuid()
    dAppId = faker.string.uuid()
    chainId = faker.string.numeric(5)
    address = faker.string.hexadecimal({ length: 40 })
    correlationId = faker.string.uuid()
    deployment = new AppDeployment({ requestId })
  })

  describe('when idle', () => {
    beforeEach(() => {
      expect(deployment.status).toBe('Idle')
    })

    describe(`on 'back:dapp:validation:requested`, () => {
      let messages: AppDeploymentEvents[]
      beforeEach(() => {
        messages = deployment.send(
          back.dappValidationRequested(
            { requestId, dAppId, chainId, address },
            { correlationId },
          ),
        )
      })

      it(`should publish 'web3:contract:validation:requested'`, () => {
        expect(messages[0].payload).toEqual({ requestId, address, chainId })
      })

      it('should propagate the metadata', () => {
        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual({ correlationId })
      })
    })
  })

  describe('when confirming', () => {
    beforeEach(() => {
      deployment.send(
        back.dappValidationRequested(
          { requestId, dAppId, address, chainId },
          { correlationId },
        ),
      )
      expect(deployment.status).toBe('Confirming')
    })

    describe(`on 'web3:contract:validation:success`, () => {
      let messages: AppDeploymentEvents[]
      beforeEach(() => {
        messages = deployment.send(
          web3.contractValidationSuccess(
            { requestId, chainId, address },
            { correlationId },
          ),
        )
      })
      it(`should publish a 'back:dapp:validation:confirmed' event`, () => {
        expect(messages.length).toBe(1)
        expect(messages[0].type).toBe('back:dapp:validation:confirmed')
      })

      it(`should propagate the right requestId`, () => {
        expect(messages.length).toBe(1)
        expect(messages[0].payload.requestId).toBe(requestId)
      })

      it(`should propagate the right dAppId`, () => {
        expect(messages.length).toBe(1)
        expect((messages[0].payload as any).dAppId).toBe(dAppId)
      })

      it('should propagate metadata', () => {
        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual({ correlationId })
      })
    })

    describe(`on 'web3:contract:validation:failure`, () => {
      let messages: AppDeploymentEvents[]
      let reason: string

      beforeEach(() => {
        reason = faker.lorem.word(5)
        messages = deployment.send(
          web3.contractValidationFailure(
            { requestId, chainId, address, reason },
            { correlationId },
          ),
        )
      })

      it(`should publish a 'back:dapp:validation:failed' event`, () => {
        expect(messages.length).toBe(1)
        expect(messages[0].type).toBe('back:dapp:validation:failed')
      })

      it('should return the right dAppId', () => {
        expect(messages.length).toBe(1)
        expect((messages[0].payload as any).dAppId).toBe(dAppId)
      })

      it('should propagate metadata', () => {
        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual({ correlationId })
      })
    })
  })
})
