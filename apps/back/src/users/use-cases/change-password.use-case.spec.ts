import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { ChangePassword } from './change-password.use-case.js'
import { TestBed } from '@suites/unit'
import { User } from '#users/domain/entities/user.js'
import { faker } from '@faker-js/faker'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import {
  IUpdateUserPassword,
  UPDATE_USER_PASSWORD,
} from './update-user-password.use-case.js'
import { Task } from 'utils'

describe('ChangePassword', () => {
  let useCase: ChangePassword
  let updateUserPassword: Mocked<IUpdateUserPassword>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(ChangePassword).compile()
    useCase = unit

    updateUserPassword = unitRef.get(
      UPDATE_USER_PASSWORD,
    ) as unknown as Mocked<IUpdateUserPassword>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given the user has logged in', () => {
    let user: User
    let password: string

    beforeEach(() => {
      password = faker.internet.password()
      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(password).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      updateUserPassword.execute.mockReturnValue(Task.of({ user }))
    })

    describe('when the old password is correct', () => {
      let newPassword: string
      let result: ReturnType<ChangePassword['execute']>
      beforeEach(() => {
        newPassword = faker.internet.password()
        result = useCase.execute(
          { oldPassword: password, newPassword },
          { user },
        )
      })

      test('then it should change the password', async () => {
        await result.toPromise()
        expect(updateUserPassword.execute).toHaveBeenCalledExactlyOnceWith(
          {
            userId: user.id.value,
            password: newPassword,
          },
          { user },
        )
      })
    })

    describe('when the old password is incorrect', () => {
      let newPassword: string
      let result: ReturnType<ChangePassword['execute']>
      beforeEach(() => {
        newPassword = faker.internet.password()
        result = useCase.execute(
          { oldPassword: faker.internet.password(), newPassword },
          { user },
        )
      })

      test('then it should throw a forbidden error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/forbidden/i)
      })

      test('then it should not udpate the password', async () => {
        try {
          await result.toPromise()
        } catch {
          // ignore
        }
        expect(updateUserPassword.execute).not.toHaveBeenCalled()
      })
    })
  })

  describe('given the user is not logged in', () => {
    let result: ReturnType<ChangePassword['execute']>

    describe('when the user tries to update the password', () => {
      let result: ReturnType<ChangePassword['execute']>
      beforeEach(() => {
        result = useCase.execute(
          {
            oldPassword: faker.internet.password(),
            newPassword: faker.internet.password(),
          },
          {},
        )
      })

      test('then it should throw an unauthorized error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/unauthorized/i)
      })

      test('then it should not update the password', async () => {
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
