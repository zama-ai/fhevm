import { Meta, StoryObj } from '@storybook/react'
import { ListApiKeys } from './list-api-keys'
import { graphql, HttpResponse } from 'msw'

let apiKeys = [
  {
    id: 'api-00001',
    name: 'Test API Key 1',
    createdAt: 1744627138231,
  },
  {
    id: 'api-00002',
    name: 'Test API Key 2',
    description: 'This API Key has a description',
    createdAt: 1744627458231,
  },
]

const meta = {
  title: 'Molecules/ListApiKeys',
  component: ListApiKeys,
  parameters: {
    layout: 'padded',
    msw: {
      handlers: [
        graphql.query(`ListApiKeys`, ({ variables }) => {
          const { dappId } = variables
          return HttpResponse.json({
            data: {
              dapp: {
                apiKeys: apiKeys.map(apiKey => ({ ...apiKey, dappId })),
              },
            },
          })
        }),
        graphql.mutation(`DeleteApiKey`, ({ variables }) => {
          console.log(`variables: ${JSON.stringify(variables)}`)
          const { apiKeyId } = variables
          apiKeys = apiKeys.filter(apiKey => apiKey.id !== apiKeyId)

          return HttpResponse.json({
            data: {
              deleteApiKey: apiKeyId,
            },
          })
        }),
      ],
    },
  },
  tags: ['autodocs'],
} satisfies Meta<typeof ListApiKeys>

export default meta
type Story = StoryObj<typeof ListApiKeys>

export const Default: Story = {
  args: {
    dappId: 'dapp-0001',
  },
}
