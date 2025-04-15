import { beforeEach, describe, expect, test } from 'vitest'
import { GetKeyUrl } from './get-key-url.use-case.js'
import {
  KEY_URL_SERVICE,
  KeyUrlService,
} from '#httpz/domain/service/key-url.service.js'
import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { Task } from 'utils'
import { faker } from '@faker-js/faker'
import { TestBed, UnitReference } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'

describe('GetKeyUrl', () => {
  let module: UnitReference
  let useCase: GetKeyUrl

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(GetKeyUrl).compile()

    useCase = unit
    module = unitRef
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('execute', () => {
    let keyUrlService: Mocked<KeyUrlService>
    beforeEach(() => {
      keyUrlService = module.get(
        KEY_URL_SERVICE,
      ) as unknown as Mocked<KeyUrlService>
      keyUrlService.getFHEPublicKey.mockReturnValue(
        Task.of([
          FHEPublicKey.parse({
            fhe_public_key: {
              data_id: faker.string.uuid(),
              urls: [faker.internet.url()],
            },
          }).unwrap(),
        ]),
      )
      keyUrlService.getCRS.mockReturnValue(
        Task.of({
          [faker.string.uuid()]: CRS.parse({
            data_id: faker.string.uuid(),
            urls: [faker.internet.url()],
          }).unwrap(),
        }),
      )
    })

    test('should return the FHE public keys', async () => {
      const { fhe_key_info } = await useCase.execute().toPromise()
      expect(fhe_key_info).toBeDefined()
      expect(fhe_key_info.length).toBeGreaterThan(0)
      expect(keyUrlService.getFHEPublicKey).toHaveBeenCalled()
    })

    test('should return the CRS URLs', async () => {
      const { crs } = await useCase.execute().toPromise()
      expect(crs).toBeDefined()
      expect(Object.keys(crs).length).toBeGreaterThan(0)
      expect(keyUrlService.getCRS).toHaveBeenCalled()
    })
  })
})
