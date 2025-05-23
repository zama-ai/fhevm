import { TestBed } from '@suites/unit'
import { beforeEach, describe, expect, test } from 'vitest'
import { CreateResetPasswordToken } from './create-reset-password-token.use-case.js'
import { User } from '#users/domain/entities/user.js'
import { faker } from '@faker-js/faker'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { Mocked } from '@suites/doubles.vitest'
import { GetUserByEmail } from '#users/use-cases/get-user-by-email.use-case.js'
import { notFoundError, Task, UnitOfWork } from 'utils'
import {
  PASSWORD_RESET_TOKEN_REPOSITORY,
  PasswordResetTokenRepository,
} from '#auth/domain/repositories/password-reset-token.repository.js'
import { PasswordResetToken } from '#auth/domain/entities/password-reset-token.js'
import { IProducer } from '#shared/services/producer.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import {
  DELETE_RESET_PASSWORD_TOKEN,
  IDeleteResetPasswordToken,
} from './delete-reset-password-token.use-case.js'

describe('CreateResetPasswordToken', () => {
  let useCase: CreateResetPasswordToken
  let repo: Mocked<PasswordResetTokenRepository>
  let producer: Mocked<IProducer>
  let getUserByEmail: Mocked<GetUserByEmail>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      CreateResetPasswordToken,
    ).compile()

    useCase = unit
    const uow = unitRef.get(UNIT_OF_WORK) as unknown as Mocked<UnitOfWork>
    uow.exec.mockImplementation(task => task)

    repo = unitRef.get(
      PASSWORD_RESET_TOKEN_REPOSITORY,
    ) as unknown as Mocked<PasswordResetTokenRepository>
    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    getUserByEmail = unitRef.get(
      GetUserByEmail,
    ) as unknown as Mocked<GetUserByEmail>

    const deleteResetPasswordToken = unitRef.get(
      DELETE_RESET_PASSWORD_TOKEN,
    ) as unknown as Mocked<IDeleteResetPasswordToken>
    deleteResetPasswordToken.execute.mockReturnValue(Task.of(void 0))
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe("given the user's email does exist", () => {
    let email: string
    let user: User

    beforeEach(() => {
      email = faker.internet.email()
      user = User.create({
        email,
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      getUserByEmail.execute.mockReturnValue(Task.of(user))
    })

    describe('when executing the use case', () => {
      let result: ReturnType<CreateResetPasswordToken['execute']>

      beforeEach(() => {
        repo.create.mockImplementation((token: PasswordResetToken) => {
          expect(token).toBeInstanceOf(PasswordResetToken)
          return Task.of(token)
        })
        producer.publish.mockReturnValue(Task.of(void 0))
        result = useCase.execute({ email }, {})
      })

      test('then it returns void', async () => {
        await expect(result.toPromise()).resolves.toBeUndefined()
      })

      test('then it call the GetUserByEmail use case', async () => {
        await result.toPromise()
        expect(getUserByEmail.execute).toHaveBeenCalledWith(email)
      })

      test('then it returns void', async () => {
        await expect(result.toPromise()).resolves.toBeUndefined()
      })

      test('then it creates a token', async () => {
        await result.toPromise()
        // NOTE: does not work for some reason
        expect(repo.create).toHaveBeenCalledOnce()
        // repo.create.lastCall()?
        // repo.create.
      })

      test('then it sends a reset password requested event', async () => {
        await result.toPromise()
        expect(producer.publish).toHaveBeenCalledExactlyOnceWith({
          type: 'back:user:password-reset:requested',
          payload: expect.objectContaining({
            requestId: expect.any(String),
            email,
            token: expect.stringMatching(/^tkn_.*$/),
          }),
          meta: {
            correlationId: expect.any(String),
          },
        })
      })
    })
  })

  describe("given the user's email does not exist", () => {
    let email: string

    beforeEach(() => {
      email = faker.internet.email()
      getUserByEmail.execute.mockReturnValue(Task.reject(notFoundError()))
    })

    describe('when executing the use case', () => {
      let result: ReturnType<CreateResetPasswordToken['execute']>
      beforeEach(() => {
        result = useCase.execute({ email }, {})
      })

      test('then it returns void', async () => {
        await expect(result.toPromise()).resolves.toBeUndefined()
      })

      test("then it doesn't create a token", async () => {
        await result.toPromise()
        expect(repo.create).not.toHaveBeenCalled()
      })

      test("then it doesn't send a reset password requested event", async () => {
        await result.toPromise()
        expect(producer.publish).not.toHaveBeenCalled()
      })
    })
  })
})
