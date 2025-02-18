import { beforeEach, describe, expect, it } from 'vitest'
import { AppDeployment, AppDeploymentEvents } from './app-deployment.js'
import { faker } from '@faker-js/faker'

import { back, web3 } from 'messages'

const chainId = '1'
const address = '0xa2dd817c2fdc3a2996f1a5174cf8f1aaed466e82'

describe('AppDeployment', () => {
  let deployment: AppDeployment
  let dAppId: string
  let correlationId: string

  beforeEach(() => {
    dAppId = faker.string.uuid()
    correlationId = faker.string.uuid()
    deployment = new AppDeployment({ chainId, address })
  })

  describe('when idle', () => {
    beforeEach(() => {
      expect(deployment.status).toBe('Idle')
    })

    describe(`on 'back:dapp:created`, () => {
      let messages: AppDeploymentEvents[]
      beforeEach(() => {
        messages = deployment.send(
          back.dappCreated({ chainId, address, dAppId }, { correlationId }),
        )
      })

      it(`should publish 'web3:contract:validation:requested'`, () => {
        expect(messages[0].payload).toEqual({ address, chainId })
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
        back.dappCreated({ dAppId, address, chainId }, { correlationId }),
      )
      expect(deployment.status).toBe('Confirming')
    })

    describe(`on 'web3:contract:validation:success`, () => {
      let messages: AppDeploymentEvents[]
      beforeEach(() => {
        messages = deployment.send(
          web3.contractValidationSuccess(
            {
              chainId,
              address,
            },
            { correlationId },
          ),
        )
      })
      it(`should publish 'back:dapp:confirmed'`, () => {
        expect(messages.length).toBe(1)
        expect(messages[0].payload).toEqual({ dAppId, address, chainId })
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
            {
              chainId,
              address,
              reason,
            },
            { correlationId },
          ),
        )
      })

      it(`should publish 'back:dapp:failed'`, () => {
        expect(messages.length).toBe(1)
        expect(messages[0].payload).toEqual({
          dAppId,
          address,
          chainId,
          reason,
        })
      })

      it('should propagate metadata', () => {
        expect(messages.length).toBe(1)
        expect(messages[0].meta).toEqual({ correlationId })
      })
    })
  })
})
