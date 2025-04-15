import { apolloClient } from '@/providers/apollo'
import { UiProvider } from '@/providers/ui'
import { ApolloProvider } from '@apollo/client'
import { faker } from '@faker-js/faker'
import { render, screen } from '@testing-library/react'
import { beforeEach, describe, expect, test } from 'vitest'
import { ListApiKeys } from './list-api-keys'

describe('ListApiKeys', () => {
  describe('given a dapp exists', () => {
    let dappId: string

    beforeEach(() => {
      dappId = faker.string.alphanumeric(10)

      render(
        <ApolloProvider client={apolloClient}>
          <UiProvider>
            <ListApiKeys dappId={dappId} />
          </UiProvider>
        </ApolloProvider>,
      )
    })

    describe('when the user request the API List', () => {
      test('then it shows a list of API Keys', async () => {
        expect(await screen.findByRole('list')).toBeInTheDocument()
      })

      test('then it shows all the API Keys', async () => {
        expect((await screen.findAllByRole('listitem')).length).toBeGreaterThan(
          0,
        )
      })
    })
  })
})
