import { beforeEach, describe, expect, test } from 'vitest'
import { CreateApiKey } from './create-api-key.use-case.js'
import { executeTask, notFoundError, Task, UnitOfWork } from 'utils'
// import { mock, MockProxy } from 'vitest-mock-extended'
import type { Mocked } from '@suites/doubles.vitest'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { ApiKeyId, DAppId } from '#dapps/domain/entities/value-objects.js'
import { DApp } from '#dapps/domain/entities/dapp.js'
import { faker } from '@faker-js/faker'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { TestBed } from '@suites/unit'
import { UNIT_OF_WORK } from '#constants.js'

describe('CreateApiKey', () => {
  let useCase: CreateApiKey
  let uow: Mocked<UnitOfWork>
  let repo: Mocked<DAppRepository>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(CreateApiKey).compile()

    uow = unitRef.get(UNIT_OF_WORK) as unknown as Mocked<UnitOfWork>
    repo = unitRef.get(DAPP_REPOSITORY) as unknown as Mocked<DAppRepository>
    useCase = unit

    uow.exec.mockImplementation(task => task)
  })

  test('should work', async () => {
    expect(useCase).toBeInstanceOf(CreateApiKey)
  })

  describe(`given a dapp exists`, () => {
    let dappId: DAppId

    beforeEach(() => {
      dappId = DAppId.random()
      repo.findById.mockReturnValue(
        Task.of(
          DApp.parse({
            id: dappId.value,
            name: faker.string.alphanumeric(10),
            status: 'LIVE',
            teamId: TeamId.random().value,
            address: faker.string.hexadecimal({ length: 40 }),
            createdAt: faker.date.past(),
          }).unwrap(),
        ),
      )
    })

    describe(`when I create a api key`, () => {
      let apiKeyId: ApiKeyId
      let name: string
      let description: string

      beforeEach(() => {
        apiKeyId = ApiKeyId.random()
        name = faker.string.alphanumeric(10)
        description = faker.lorem.sentence()

        repo.createApiKey.mockReturnValue(
          Task.of(
            ApiKey.parse({
              id: apiKeyId.value,
              dappId: dappId.value,
              name,
              description,
            }).unwrap(),
          ),
        )
      })

      test(`then it should succeed`, async () => {
        const { success, value, error } = await executeTask(
          useCase.execute({
            dappId: dappId.value,
            name,
            description,
          }),
        )
        if (!success) {
          console.log(`test failed: ${error._tag}/${error.message}`)
        }
        expect(success, 'Create api key should succeed').toBe(true)
        expect(value).toBeDefined()
        expect(value?.dappId.value).toBe(dappId.value)
        expect(value?.id.value).toBe(apiKeyId.value)
        expect(value?.name).toBe(name)
      })
    })

    // TODO: conver the authorization logic
    describe.skip(`when a user try to create an api key for a dapp they don't own`, () => {})
  })

  describe(`given a dapp doesn't exist`, () => {
    let dappId: DAppId

    beforeEach(() => {
      dappId = DAppId.random()
      repo.findById.mockReturnValue(
        Task.reject(notFoundError(`DApp with id ${dappId.value} not found`)),
      )
    })

    describe(`when I create a api key`, () => {
      let apiKeyId: ApiKeyId
      let name: string
      let description: string

      beforeEach(() => {
        apiKeyId = ApiKeyId.random()
        name = faker.string.alphanumeric(10)
        description = faker.lorem.sentence()

        repo.createApiKey.mockReturnValue(
          Task.of(
            ApiKey.parse({
              id: apiKeyId.value,
              dappId: dappId.value,
              name,
              description,
            }).unwrap(),
          ),
        )
      })

      test(`then it should fail`, async () => {
        const { success, error } = await executeTask(
          useCase.execute({
            dappId: dappId.value,
            name,
            description,
          }),
        )
        expect(success).toBe(false)
        expect(error).toBeDefined()
        expect(error?._tag).toBe('NotFoundError')
      })
    })
  })
})
