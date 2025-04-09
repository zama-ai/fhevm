import { graphql, HttpResponse } from 'msw'
import { faker } from '@faker-js/faker'

export const handlers = [
  graphql.mutation(`CreateApiKey`, ({ variables }) => {
    const { name, description, dappId } = variables

    return HttpResponse.json({
      data: {
        createApiKey: {
          token: `pk-${faker.string.nanoid(10)}`,
          apiKey: {
            id: faker.string.nanoid(),
            name,
            description,
            dappId,
          },
        },
      },
    })
  }),
  graphql.query(`ListApiKeys`, ({ variables }) => {
    const { dappId } = variables
    return HttpResponse.json({
      data: {
        dapp: {
          apiKeys: [
            {
              id: faker.string.uuid(),
              name: faker.string.alphanumeric(10),
              description: null,
              dappId,
            },
            {
              id: faker.string.uuid(),
              name: faker.string.alphanumeric(10),
              description: faker.lorem.paragraph(),
              dappId,
            },
          ],
        },
      },
    })
  }),
  graphql.mutation(`DeleteApiKey`, ({ variables }) => {
    console.log(`variables: ${JSON.stringify(variables)}`)
    const { apiKeyId } = variables

    return HttpResponse.json({
      data: {
        deleteApiKey: apiKeyId,
      },
    })
  }),
]
