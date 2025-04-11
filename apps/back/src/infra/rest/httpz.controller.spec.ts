import { beforeEach, describe, expect, test } from 'vitest'
import { HttpzController } from './httpz.controller.js'
import { Test, TestingModule } from '@nestjs/testing'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Task } from 'utils'
import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { faker } from '@faker-js/faker'
import { mock, MockProxy } from 'vitest-mock-extended'
import { InputProof } from '#httpz/use-cases/input-proof.use-case.js'

describe('HttpzController', () => {
  let module: TestingModule
  let controller: HttpzController

  beforeEach(async () => {
    module = await Test.createTestingModule({
      controllers: [HttpzController],
      providers: [
        {
          provide: GetKeyUrl,
          useValue: mock(),
        },
        {
          provide: InputProof,
          useValue: mock(),
        },
      ],
    }).compile()

    controller = module.get(HttpzController)
  })

  test('should be defined', () => {
    expect(controller).toBeDefined()
  })

  describe('GET /v1/keyurl', () => {
    let getKeyUrl: MockProxy<GetKeyUrl>
    beforeEach(() => {
      getKeyUrl = module.get(GetKeyUrl)
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

  describe(`POST /v1/input-proof`, () => {
    let inputProof: MockProxy<InputProof>
    let handles: string[]
    let signatures: string[]

    beforeEach(() => {
      handles = [faker.string.hexadecimal({ length: 40, prefix: '' })]
      signatures = [faker.string.hexadecimal({ length: 40 })]
      inputProof = module.get(InputProof)
      inputProof.execute.mockReturnValue(Task.of({ handles, signatures }))
    })

    test('should return a success response', async () => {
      await controller.postInputProof({
        contractChainId: faker.string.hexadecimal({ length: 3 }),
        contractAddress: faker.string.hexadecimal({ length: 40 }),
        userAddress: faker.string.hexadecimal({ length: 40 }),
        ciphertextWithZkpok: faker.string.hexadecimal({ length: 40 }),
      })
      expect(inputProof.execute).toHaveBeenCalledOnce()
    })

    test('should return the handles and the signatures', async () => {
      const response = await controller.postInputProof({
        contractChainId: faker.string.hexadecimal({ length: 3 }),
        contractAddress: faker.string.hexadecimal({ length: 40 }),
        userAddress: faker.string.hexadecimal({ length: 40 }),
        ciphertextWithZkpok: faker.string.hexadecimal({ length: 40 }),
      })
      expect(response).toEqual({ response: { handles, signatures } })
    })
  })
})
