import { render, screen } from '@testing-library/react'
import { beforeEach, describe, expect, test } from 'vitest'
import { ChangePassword } from './change-password'
import { UiProvider } from '@/providers/ui'
import userEvent from '@testing-library/user-event'
import { faker } from '@faker-js/faker'
import { server } from '@/setup'
import { graphql, HttpResponse } from 'msw'
import { ApolloProvider } from '@apollo/client'
import { apolloClient } from '@/providers/apollo'
import { ChangePasswordSchema } from './schema'

describe('ChangePassword', () => {
  beforeEach(() => {
    render(
      <ApolloProvider client={apolloClient}>
        <UiProvider>
          <ChangePassword />
        </UiProvider>
      </ApolloProvider>,
    )
  })

  test('should work', async () => {
    expect(await screen.findByRole('form')).toBeInTheDocument()
  })

  describe('view mode', () => {
    test('should show a edit button', async () => {
      expect(
        await screen.findByRole('button', { name: 'Edit' }),
      ).toBeInTheDocument()
    })

    describe('when the user click on edit', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        await user.click(await screen.findByRole('button', { name: 'Edit' }))
      })

      test('then it shows a cancel button', async () => {
        expect(
          await screen.findByRole('button', { name: 'Cancel' }),
        ).toBeInTheDocument()
      })
    })
  })

  describe('edit mode', () => {
    beforeEach(async () => {
      const user = userEvent.setup()
      await user.click(await screen.findByRole('button', { name: 'Edit' }))
    })

    test('should show a old password field', async () => {
      expect(await screen.findByLabelText('Old Password')).toBeInTheDocument()
    })

    test('should show a new password field', async () => {
      expect(await screen.findByLabelText('New Password')).toBeInTheDocument()
    })

    test('should show a repeat password field', async () => {
      expect(
        await screen.findByLabelText('Repeat Password'),
      ).toBeInTheDocument()
    })

    test('should show a save button', async () => {
      expect(
        await screen.findByRole('button', { name: 'Save' }),
      ).toBeInTheDocument()
    })

    describe('when the user click on cancel', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        await user.click(await screen.findByRole('button', { name: 'Cancel' }))
      })

      test('then it shows a edit button', async () => {
        expect(
          await screen.findByRole('button', { name: 'Edit' }),
        ).toBeInTheDocument()
      })
    })

    describe('when the user fills the form correctly', () => {
      let oldPassword: string
      let newPassword: string

      beforeEach(async () => {
        const user = userEvent.setup()

        oldPassword = faker.internet.password({ length: 8 })
        do {
          newPassword = faker.internet.password({ length: 15 })
        } while (
          !ChangePasswordSchema.safeParse({
            oldPassword,
            newPassword,
            repeatPassword: newPassword,
          }).success
        )
        await user.type(
          await screen.findByLabelText('Old Password'),
          oldPassword,
        )
        await user.type(
          await screen.findByLabelText('New Password'),
          newPassword,
        )
        await user.type(
          await screen.findByLabelText('Repeat Password'),
          newPassword,
        )

        server.use(
          graphql.mutation('ChangePassword', ({ variables }) => {
            if (!variables.oldPassword || !variables.oldPassword) {
              return HttpResponse.json({
                data: null,
                errors: [{ message: 'Validation Error' }],
              })
            }
            return HttpResponse.json({
              data: {
                changePassword: true,
              },
            })
          }),
        )

        await user.click(await screen.findByRole('button', { name: 'Save' }))
      })

      test('then it updates the password', async () => {
        const alert = await screen.findByRole('alert')
        expect(alert).toBeInTheDocument()
        expect(alert).toContainHTML('Your password has been updated')
      })

      test('then it shows the edit button', async () => {
        expect(
          await screen.findByRole('button', { name: 'Edit' }),
        ).toBeInTheDocument()
      })
    })

    describe('when the server returns an error', () => {
      let error: string

      beforeEach(async () => {
        error = faker.lorem.words(5)
        const user = userEvent.setup()

        const oldPassword = faker.internet.password({ length: 8 })
        let newPassword: string
        do {
          newPassword = faker.internet.password({ length: 15 })
        } while (
          !ChangePasswordSchema.safeParse({
            oldPassword,
            newPassword,
            repeatPassword: newPassword,
          }).success
        )
        await user.type(
          await screen.findByLabelText('Old Password'),
          oldPassword,
        )
        await user.type(
          await screen.findByLabelText('New Password'),
          newPassword,
        )
        await user.type(
          await screen.findByLabelText('Repeat Password'),
          newPassword,
        )

        server.use(
          graphql.mutation('ChangePassword', () => {
            return HttpResponse.json({
              data: null,
              errors: [{ message: error }],
            })
          }),
        )

        await user.click(await screen.findByRole('button', { name: 'Save' }))
      })

      test('then it shows the error', async () => {
        const alert = await screen.findByRole('alert')
        expect(alert).toBeInTheDocument()
        expect(alert).toContainHTML(error)
      })

      test('then it shows the cancel button', async () => {
        expect(
          await screen.findByRole('button', { name: 'Cancel' }),
        ).toBeInTheDocument()
      })
    })

    describe('when the user does not fill the old password', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        const password = faker.internet.password()
        await user.type(await screen.findByLabelText('New Password'), password)
        await user.type(
          await screen.findByLabelText('Repeat Password'),
          password,
        )
        await user.click(await screen.findByRole('button', { name: 'Save' }))
      })
      test('then it shows an error', async () => {
        // const alerts = await screen.findAllByRole('alert')
        // console.log(alerts[0].title)

        expect(
          await screen.findByText('Old password is required'),
        ).toBeInTheDocument()
      })
    })

    describe('when the user does fill an invalid new password', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        // it should fail due a missing special char
        const password = faker.string.alphanumeric(15)
        await user.type(
          await screen.findByLabelText('Old Password'),
          faker.internet.password(),
        )
        await user.type(await screen.findByLabelText('New Password'), password)
        await user.type(
          await screen.findByLabelText('Repeat Password'),
          password,
        )
        await user.click(await screen.findByRole('button', { name: 'Save' }))
      })

      test('then it shows an error', async () => {
        expect(
          await screen.findByText(
            'Password should contain at least a special character',
          ),
        ).toBeInTheDocument()
      })
    })
  })
})
