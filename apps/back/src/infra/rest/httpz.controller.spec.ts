import { beforeEach, describe, expect, test } from 'vitest'
import { HttpzController } from './httpz.controller.js'
import { TestBed, UnitReference } from '@suites/unit'
import type { Mocked } from '@suites/doubles.vitest'
// import { Test, TestingModule } from '@nestjs/testing'
import { Task } from 'utils'
import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { faker } from '@faker-js/faker'
import { GetKeyUrl, InputProof } from '#httpz/use-cases/index.js'
// import { ApiKeyAllowsRequest, GetApiKey } from '#dapps/use-cases/index.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { ApiKeyId, DAppId } from '#dapps/domain/entities/value-objects.js'

describe('HttpzController', () => {
  // let module: TestingModule
  let unitRef: UnitReference
  let controller: HttpzController

  beforeEach(async () => {
    const { unit, unitRef: _unitRef } =
      await TestBed.solitary(HttpzController).compile()
    controller = unit
    unitRef = _unitRef
  })

  test('should be defined', () => {
    expect(controller).toBeDefined()
  })

  describe('GET /keyurl', () => {
    let getKeyUrl: Mocked<GetKeyUrl>
    beforeEach(() => {
      getKeyUrl = unitRef.get(GetKeyUrl) as unknown as Mocked<GetKeyUrl>
      getKeyUrl.execute.mockReturnValue(
        Task.of({
          fhe_key_info: [
            FHEPublicKey.parse({
              fhe_public_key: {
                data_id: faker.string.uuid(),
                urls: [faker.internet.url()],
              },
            }).unwrap(),
          ],
          crs: {
            [faker.string.uuid()]: CRS.parse({
              data_id: faker.string.uuid(),
              urls: [faker.internet.url()],
            }).unwrap(),
          },
        }),
      )
    })

    test('should return the FHE public keys', async () => {
      const {
        response: { fhe_key_info },
      } = await controller.getKeyUrl()
      expect(fhe_key_info).toBeDefined()
      expect(fhe_key_info.length).toBeGreaterThan(0)
      expect(getKeyUrl.execute.mock.calls.length).toBe(1)
    })

    test('should return the CRS URLs', async () => {
      const {
        response: { crs },
      } = await controller.getKeyUrl()
      expect(crs).toBeDefined()
      expect(Object.keys(crs).length).toBeGreaterThan(0)
      expect(getKeyUrl.execute.mock.calls.length).toBe(1)
    })
  })

  describe(`POST /input-proof`, () => {
    let inputProof: Mocked<InputProof>
    let handles: string[]
    let signatures: string[]
    let apiKey: ApiKey

    beforeEach(() => {
      handles = [faker.string.hexadecimal({ length: 40, prefix: '' })]
      signatures = [faker.string.hexadecimal({ length: 40 })]
      inputProof = unitRef.get(InputProof) as unknown as Mocked<InputProof>
      inputProof.execute.mockReturnValue(Task.of({ handles, signatures }))
      apiKey = ApiKey.parse({
        id: ApiKeyId.random().value,
        dappId: DAppId.random().value,
        name: faker.string.alphanumeric(10),
      }).unwrap()
    })

    test('should return a success response', async () => {
      await controller.postInputProof(apiKey, {
        contractChainId: faker.string.hexadecimal({ length: 3 }),
        contractAddress: faker.string.hexadecimal({ length: 40 }),
        userAddress: faker.string.hexadecimal({ length: 40 }),
        ciphertextWithZkpok: faker.string.hexadecimal({ length: 40 }),
      })
      expect(inputProof.execute).toHaveBeenCalledOnce()
    })

    test('should return the handles and the signatures', async () => {
      const response = await controller.postInputProof(apiKey, {
        contractChainId: faker.string.hexadecimal({ length: 3 }),
        contractAddress: faker.string.hexadecimal({ length: 40 }),
        userAddress: faker.string.hexadecimal({ length: 40 }),
        ciphertextWithZkpok: faker.string.hexadecimal({ length: 40 }),
      })
      expect(response).toEqual({ response: { handles, signatures } })
    })
  })
})
