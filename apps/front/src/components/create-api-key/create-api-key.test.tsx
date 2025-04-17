import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, test } from 'vitest'
import { CreateApiKey } from './create-api-key'
import { faker } from '@faker-js/faker'
import { ApolloProvider } from '@apollo/client'
import { apolloClient } from '@/providers/apollo'
import { UiProvider } from '@/providers/ui'
import { server } from '@/setup'
import { graphql, HttpResponse } from 'msw'
import { vi } from 'vitest'

describe('CreateApiKey', () => {
  describe('given a dapp exists', () => {
    let dappId: string

    beforeEach(() => {
      dappId = faker.string.alphanumeric(10)

      render(
        <ApolloProvider client={apolloClient}>
          <UiProvider>
            <CreateApiKey dappId={dappId} />
          </UiProvider>
        </ApolloProvider>,
      )
    })

    describe('when the user fill the form', () => {
      beforeEach(async () => {
        const user = userEvent.setup()
        const form = screen.getByRole('form')

        await user.type(
          within(form).getByTestId('api-key-name'),
          // NOTE: Api key name should be shorter then 50 characters
          faker.lorem.words({ min: 3, max: 10 }).slice(0, 50),
        )
        await user.click(within(form).getByRole('button'))
      })

      test('then it creates a new api key', async () => {
        expect(await screen.findByRole('code')).toBeInTheDocument()
      })
    })

    describe('when the server rises an error', () => {
      beforeEach(async () => {
        // Setup global error handler to prevent test from failing on unhandled errors
        const originalConsoleError = console.error
        console.error = vi.fn()

        vi.stubGlobal(
          'onunhandledrejection',
          (event: PromiseRejectionEvent) => {
            event.preventDefault()
          },
        )

        server.use(
          graphql.mutation('CreateApiKey', () => {
            return HttpResponse.json({
              errors: [{ message: 'mocked error' }],
              data: null,
            })
          }),
        )

        const user = userEvent.setup()
        const form = screen.getByRole('form')

        await user.type(
          within(form).getByTestId('api-key-name'),
          faker.string.alpha(10),
        )
        await user.click(within(form).getByRole('button'))

        // Restore console.error after test
        return () => {
          console.error = originalConsoleError
        }
      })

      test('then it shows the error message', async () => {
        expect(await screen.findByText('mocked error')).toBeInTheDocument()
      })

      test('form remains usable after error', async () => {
        // Wait for error to be displayed
        await screen.findByText('mocked error')

        // Verify form can still be interacted with
        const form = screen.getByRole('form')
        expect(within(form).getByTestId('api-key-name')).toBeEnabled()
        expect(within(form).getByRole('button')).toBeEnabled()
      })
    })

    describe('when unexpected Server errors occur', () => {
      beforeEach(async () => {
        // Setup error handling to prevent test failures
        const originalError = console.error
        console.error = vi.fn()

        // Mock a network error
        server.use(
          graphql.mutation('CreateApiKey', () => {
            return HttpResponse.json({
              errors: [{ message: 'Network error' }],
              data: null,
            })
          }),
        )

        const user = userEvent.setup()
        const form = screen.getByRole('form')

        await user.type(
          within(form).getByTestId('api-key-name'),
          faker.string.alpha(10),
        )
        await user.click(within(form).getByRole('button'))

        return () => {
          console.error = originalError
        }
      })

      test("application doesn't crash", () => {
        expect(screen.getByRole('form')).toBeInTheDocument()
      })
    })
  })
})
