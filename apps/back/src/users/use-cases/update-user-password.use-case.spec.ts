import { TestBed } from '@suites/unit'
import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import {
  UpdateUserPassword,
  UpdateUserPasswordWithAuthorization,
} from './update-user-password.use-case.js'
import { User } from '#users/domain/entities/user.js'
import { faker } from '@faker-js/faker'
import {
  UserId,
  ValidatedPassword,
} from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { AppError, notFoundError, Task } from 'utils'

describe('UpdateUserPassword', () => {
  let useCase: UpdateUserPassword
  let repo: Mocked<UserRepository>

  beforeEach(async () => {
    const { unit, unitRef } =
      await TestBed.solitary(UpdateUserPassword).compile()

    useCase = unit
    repo = unitRef.get(USER_REPOSITORY) as unknown as Mocked<UserRepository>
  })

  test('it should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given the user exists', () => {
    let user: User
    let password: string

    beforeEach(() => {
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()
      password = faker.internet.password()

      repo.findById.mockReturnValue(Task.of(user))
      repo.update.mockImplementation((userId, props) => {
        expect(userId.value).toBe(user.id.value)
        // NOTE: props.password is an hash
        const newUser = User.parse({
          ...user.toJSON(),
          password: props?.password ?? '',
        }).unwrap()

        expect(
          newUser.checkPassword(password).isOk(),
          'Failed to check password',
        ).toBe(true)
        return Task.of<User, AppError>(user)
      })
    })

    describe('when called', () => {
      let result: ReturnType<UpdateUserPassword['execute']>

      beforeEach(() => {
        result = useCase.execute({ userId: user.id.value, password }, { user })
      })

      test('then it should update the password with its hash', async () => {
        await result.toPromise()
        // NOTE: I check the hash in the mock implementation method, because
        // every time I hash it, the value is different due to the injected salt.
        expect(repo.update).toHaveBeenCalledOnce()
      })

      test('then it should return the user', async () => {
        const response = await result.toPromise()
        expect(response.user).toEqual(user)
      })
    })
  })

  describe('given the user does not exist', () => {
    let userId: string

    beforeEach(() => {
      userId = UserId.random().value
      repo.findById.mockReturnValue(Task.reject(notFoundError()))
    })

    describe('when called', () => {
      let result: ReturnType<UpdateUserPassword['execute']>

      beforeEach(() => {
        result = useCase.execute(
          { userId, password: faker.internet.password() },
          {},
        )
      })

      test('then it should throw a not found error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/not found/i)
      })

      test('then it should not update the password', async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(repo.update).not.toHaveBeenCalled()
      })
    })
  })
})

describe('UpdateUserPasswordWithAuthorization', () => {
  let useCase: UpdateUserPasswordWithAuthorization
  let updateUserPassword: Mocked<UpdateUserPassword>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      UpdateUserPasswordWithAuthorization,
    ).compile()

    useCase = unit
    updateUserPassword = unitRef.get(
      UpdateUserPassword,
    ) as unknown as Mocked<UpdateUserPassword>
  })

  test('it should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given a user exists', () => {
    let userId: string
    beforeEach(() => {
      userId = UserId.random().value
    })

    describe('when a user update its own password', () => {
      let user: User
      let password: string
      let result: ReturnType<UpdateUserPasswordWithAuthorization['execute']>

      beforeEach(() => {
        user = User.create({
          email: faker.internet.email(),
          password: ValidatedPassword.validate(
            faker.internet.password(),
          ).unwrap(),
          name: faker.person.fullName(),
        }).unwrap()

        password = faker.internet.password()

        updateUserPassword.execute.mockReturnValue(Task.of({ user }))
        result = useCase.execute({ userId: user.id.value, password }, { user })
      })

      test('then it should update the password', async () => {
        await result.toPromise()
        expect(updateUserPassword.execute).toHaveBeenCalledExactlyOnceWith(
          {
            userId: user.id.value,
            password,
          },
          { user },
        )
      })
    })

    describe('when the user tries to update someone else password', () => {
      let current: User
      let result: ReturnType<UpdateUserPasswordWithAuthorization['execute']>

      beforeEach(() => {
        current = User.create({
          email: faker.internet.email(),
          password: ValidatedPassword.validate(
            faker.internet.password(),
          ).unwrap(),
          name: faker.person.fullName(),
        }).unwrap()

        result = useCase.execute(
          { userId, password: faker.internet.password() },
          { user: current },
        )
      })

      test('then it should throw a forbidden error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/forbidden/i)
      })

      test('then it should not call update password', async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(updateUserPassword.execute).not.toHaveBeenCalled()
      })
    })
  })

  describe("given a user doesn't exist", () => {
    describe('when called', () => {
      let result: ReturnType<UpdateUserPasswordWithAuthorization['execute']>
      beforeEach(() => {
        result = useCase.execute(
          {
            userId: UserId.random().value,
            password: faker.internet.password(),
          },
          {},
        )
      })

      test('then it should throw a unauthorized error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/unauthorized/i)
      })
    })
  })
})
