import { beforeEach, describe, expect, test } from 'vitest'
import { HttpzController } from './httpz.controller.js'
import { Test, TestingModule } from '@nestjs/testing'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Task } from 'utils'
import { FHEPublicKey } from '#httpz/domain/entities/fhe-public-key.js'
import { faker } from '@faker-js/faker'
import { CRS } from '#httpz/domain/entities/crs.js'
import { mock, MockProxy } from 'vitest-mock-extended'

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
      ],
    }).compile()

    controller = module.get(HttpzController)
  })

  test('should be defined', () => {
    expect(controller).toBeDefined()
  })

  describe('GET /key-url', () => {
    let getKeyUrl: MockProxy<GetKeyUrl>
    beforeEach(() => {
      getKeyUrl = module.get(GetKeyUrl)
      getKeyUrl.execute.mockReturnValue(
        Task.of({
          fheKeyInfo: [
            FHEPublicKey.parse({
              dataId: faker.string.uuid(),
              urls: [faker.internet.url()],
            }).unwrap(),
          ],
          crs: [
            CRS.parse({
              dataId: faker.string.uuid(),
              urls: [faker.internet.url()],
            }).unwrap(),
          ],
        }),
      )
    })

    test('should return the FHE public keys', async () => {
      const { fheKeyInfo } = await controller.getKeyUrl()
      expect(fheKeyInfo).toBeDefined()
      expect(fheKeyInfo.length).toBeGreaterThan(0)
      expect(getKeyUrl.execute.mock.calls.length).toBe(1)
    })

    test('should return the CRS URLs', async () => {
      const { crs } = await controller.getKeyUrl()
      expect(crs).toBeDefined()
      expect(crs.length).toBeGreaterThan(0)
      expect(getKeyUrl.execute.mock.calls.length).toBe(1)
    })
  })
})
