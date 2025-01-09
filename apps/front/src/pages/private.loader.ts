import { apolloClient } from '@/providers/apollo.js'
import { graphql } from '../__generated__/gql.js'
import { MeQuery } from '@/__generated__/graphql.js'

export const GET_ME = graphql(`
  query Me {
    me {
      id
      email
      name
      teams {
        id
        name
        dapps {
          id
          name
          status
        }
      }
    }
  }
`)

export async function privateLoader() {
  const { error, data } = await apolloClient.query<MeQuery>({
    query: GET_ME,
  })
  if (error) {
    throw error
  }

  return data
}
