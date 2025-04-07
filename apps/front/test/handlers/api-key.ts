import { graphql, HttpResponse } from 'msw'
import { faker } from '@faker-js/faker'

export const handlers = [
  graphql.mutation(`createApiKey`, ({ variables }) => {
    const { name, description, dappId } = variables

    return HttpResponse.json({
      data: {
        createApiKey: {
          id: faker.string.nanoid(),
          name,
          description,
          dappId,
        },
      },
    })
  }),
]
