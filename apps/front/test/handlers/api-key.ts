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
]
