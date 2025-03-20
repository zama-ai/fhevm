import { Test, TestingModule } from '@nestjs/testing'
import { beforeEach, describe, expect, test } from 'vitest'
import { GetKeyUrl } from './get-key-url.use-case.js'
import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { Task } from 'utils'
import { faker } from '@faker-js/faker'
import { mock, MockProxy } from 'vitest-mock-extended'

describe('GetKeyUrl', () => {
  let module: TestingModule
  let useCase: GetKeyUrl

  beforeEach(async () => {
    module = await Test.createTestingModule({
      providers: [GetKeyUrl, { provide: KeyUrlService, useValue: mock() }],
    }).compile()

    useCase = module.get<GetKeyUrl>(GetKeyUrl)
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('execute', () => {
    let keyUrlService: MockProxy<KeyUrlService>
    beforeEach(() => {
      keyUrlService = module.get<KeyUrlService>(
        KeyUrlService,
      ) as MockProxy<KeyUrlService>
      keyUrlService.getFHEPublicKey.mockReturnValue(
        Task.of([
          FHEPublicKey.parse({
            dataId: faker.string.uuid(),
            urls: [faker.internet.url()],
          }).unwrap(),
        ]),
      )
      keyUrlService.getCRS.mockReturnValue(
        Task.of([
          CRS.parse({
            dataId: faker.string.uuid(),
            urls: [faker.internet.url()],
          }).unwrap(),
        ]),
      )
    })

    test('should return the FHE public keys', async () => {
      const { fheKeyInfo } = await useCase.execute().toPromise()
      expect(fheKeyInfo).toBeDefined()
      expect(fheKeyInfo.length).toBeGreaterThan(0)
      expect(keyUrlService.getFHEPublicKey).toHaveBeenCalled()
    })

    test('should return the CRS URLs', async () => {
      const { crs } = await useCase.execute().toPromise()
      expect(crs).toBeDefined()
      expect(crs.length).toBeGreaterThan(0)
      expect(keyUrlService.getCRS).toHaveBeenCalled()
    })
  })
})
