import { TestBed } from '@suites/unit'
import { beforeEach, describe, expect, test } from 'vitest'
import {
  ResetPassword,
  ResetPasswordWithEvents,
} from './reset-password.use-case.js'
import { Mocked } from '@suites/doubles.vitest'
import {
  PASSWORD_RESET_TOKEN_REPOSITORY,
  PasswordResetTokenRepository,
} from '#auth/domain/repositories/password-reset-token.repository.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { PasswordResetToken } from '#auth/domain/entities/password-reset-token.js'
import { User } from '#users/domain/entities/user.js'
import { faker } from '@faker-js/faker'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { ExpiresAt } from '#shared/entities/value-objects/expires-at.js'
import {
  IUpdateUserPassword,
  UPDATE_USER_PASSWORD,
} from '#users/use-cases/update-user-password.use-case.js'
import { notFoundError, Task, UnitOfWork, unknownError } from 'utils'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import {
  DELETE_RESET_PASSWORD_TOKEN,
  IDeleteResetPasswordToken,
} from './delete-reset-password-token.use-case.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'

describe('ResetPassword', () => {
  let useCase: ResetPassword
  let uow: Mocked<UnitOfWork>
  let repo: Mocked<PasswordResetTokenRepository>
  let getUserById: Mocked<GetUserById>
  let updateUserPassword: Mocked<IUpdateUserPassword>
  let deleteResetPasswordToken: Mocked<IDeleteResetPasswordToken>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(ResetPassword).compile()

    useCase = unit
    uow = unitRef.get(UNIT_OF_WORK) as unknown as Mocked<UnitOfWork>
    repo = unitRef.get(
      PASSWORD_RESET_TOKEN_REPOSITORY,
    ) as unknown as Mocked<PasswordResetTokenRepository>
    getUserById = unitRef.get(GetUserById) as unknown as Mocked<GetUserById>
    updateUserPassword = unitRef.get(
      UPDATE_USER_PASSWORD,
    ) as unknown as Mocked<IUpdateUserPassword>
    deleteResetPasswordToken = unitRef.get(
      DELETE_RESET_PASSWORD_TOKEN,
    ) as unknown as Mocked<IDeleteResetPasswordToken>

    uow.exec.mockImplementation(task => task)
  })

  test('it should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given a valid reset token exists', () => {
    let token: Token
    let hash: Hash
    let user: User

    beforeEach(() => {
      token = Token.random()
      hash = Hash.hash(token)
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      repo.findByHash.mockReturnValue(
        Task.of(
          PasswordResetToken.parse({
            hash: hash.value,
            userId: user.id.value,
            expiresAt: ExpiresAt.compute().value,
          }).unwrap(),
        ),
      )
      getUserById.execute.mockReturnValue(Task.of(user))
      deleteResetPasswordToken.execute.mockReturnValue(Task.of(void 0))
      updateUserPassword.execute.mockReturnValue(Task.of({ user }))
    })

    describe('when resetting the password', () => {
      let result: ReturnType<ResetPassword['execute']>
      let password: string

      beforeEach(() => {
        password = faker.internet.password()
        result = useCase.execute({ token: token.value, password }, { user })
      })

      test('then it should search for the token', async () => {
        await result.toPromise()
        expect(repo.findByHash).toHaveBeenCalledWith(Hash.hash(token))
      })

      test('then it should search for the user', async () => {
        await result.toPromise()
        expect(getUserById.execute).toHaveBeenCalledWith(user.id.value)
      })

      test('then it should delete the used token', async () => {
        await result.toPromise()
        expect(deleteResetPasswordToken.execute).toHaveBeenCalledWith({ hash })
      })

      test('then it should update the user password', async () => {
        await result.toPromise()
        expect(updateUserPassword.execute).toHaveBeenCalledWith(
          {
            userId: user.id.value,
            password,
          },
          { user },
        )
      })
    })
  })

  describe('given an expired token exists', () => {
    let token: Token
    let hash: Hash
    let user: User

    beforeEach(() => {
      token = Token.random()
      hash = Hash.hash(token)
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      repo.findByHash.mockReturnValue(
        Task.of(
          PasswordResetToken.parse({
            hash: hash.value,
            userId: user.id.value,
            expiresAt: ExpiresAt.from(faker.date.recent({ days: 10 })).unwrap()
              .value,
          }).unwrap(),
        ),
      )
      updateUserPassword.execute.mockReturnValue(Task.of({ user }))
    })

    describe('when resetting the password', () => {
      let result: ReturnType<ResetPassword['execute']>
      let password: string

      beforeEach(() => {
        password = faker.internet.password()
        result = useCase.execute({ token: token.value, password }, { user })
      })

      test('then it should throw a token expired error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(
          /token is expired/i,
        )
      })

      test("then it shouldn't update the user password", async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(updateUserPassword.execute).not.toHaveBeenCalled()
      })
    })
  })

  describe('given a token does not exist', () => {
    let token: Token
    let user: User

    beforeEach(() => {
      token = Token.random()
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      repo.findByHash.mockReturnValue(
        Task.reject(notFoundError('token not found')),
      )
      updateUserPassword.execute.mockReturnValue(Task.of({ user }))
    })

    describe('when resetting the password', () => {
      let result: ReturnType<ResetPassword['execute']>
      let password: string

      beforeEach(() => {
        password = faker.internet.password()
        result = useCase.execute({ token: token.value, password }, { user })
      })

      test('then it should throw a forbidden error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/forbidden/i)
      })

      test('then it should not update the user password', async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(updateUserPassword.execute).not.toHaveBeenCalled()
      })
    })
  })
})

describe('ResetPasswordWithEvents', () => {
  let useCase: ResetPasswordWithEvents
  let resetPassword: Mocked<ResetPassword>
  let producer: Mocked<IProducer>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      ResetPasswordWithEvents,
    ).compile()

    useCase = unit

    resetPassword = unitRef.get(
      ResetPassword,
    ) as unknown as Mocked<ResetPassword>
    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    producer.publish.mockReturnValue(Task.of(void 0))
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given reset password is successful', () => {
    let token: Token
    // let hash: Hash
    let user: User

    beforeEach(() => {
      token = Token.random()
      // hash = Hash.hash(token)
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      resetPassword.execute.mockReturnValue(Task.of({ user }))
    })

    describe('when resetting the password', () => {
      let result: ReturnType<ResetPasswordWithEvents['execute']>
      let password: string

      beforeEach(() => {
        resetPassword.execute.mockReturnValue(Task.of({ user }))
        password = faker.internet.password()
        result = useCase.execute({ token: token.value, password }, { user })
      })

      test('then it should reset the password', async () => {
        await result.toPromise()
        expect(resetPassword.execute).toHaveBeenCalledWith(
          {
            token: token.value,
            password,
          },
          { user },
        )
      })

      test('then it generates a reset password completed event', async () => {
        await result.toPromise()
        expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
          expect.objectContaining({
            type: 'back:user:password-reset:completed',
          }),
        )
      })
    })

    describe('when failing to reset the password', () => {
      let result: ReturnType<ResetPasswordWithEvents['execute']>
      let password: string

      beforeEach(() => {
        resetPassword.execute.mockReturnValue(
          Task.reject(unknownError('mock error')),
        )
        password = faker.internet.password()
        result = useCase.execute({ token: token.value, password }, { user })
      })

      test('then it should not the reset password', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/mock error/i)
      })

      test('then it not generates a reset password completed event', async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(producer.publish).not.toHaveBeenCalled()
      })
    })
  })
})
