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
          within(form).getByLabelText(/name/i),
          faker.lorem.words({ min: 1, max: 3 }),
        )
        await user.type(
          within(form).getByLabelText(/description/i),
          faker.lorem.paragraph(),
        )
        await user.click(within(form).getByRole('button'))
      })

      test('then it creates a new api key', async () => {
        expect(await screen.findByText(/api key created/i)).toBeInTheDocument()
      })
    })

    describe('when the server rises an error', () => {
      beforeEach(async () => {
        server.use(
          graphql.mutation('CreateApiKey', () => {
            return HttpResponse.json({
              errors: [{ message: 'mocked error' }],
            })
          }),
        )

        const user = userEvent.setup()

        const form = screen.getByRole('form')

        await user.type(
          within(form).getByLabelText(/name/i),
          faker.lorem.words({ min: 1, max: 3 }),
        )
        await user.type(
          within(form).getByLabelText(/description/i),
          faker.lorem.paragraph(1),
        )
        await user.click(within(form).getByRole('button'))
      })

      test('then it shows it', async () => {
        expect(await screen.findByText('mocked error')).toBeInTheDocument()
      })
    })
  })
})
