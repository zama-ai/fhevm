import { apolloClient } from '@/providers/apollo'
import { graphql } from '../__generated__/gql'
import { MeQuery } from '@/__generated__/graphql'

const GET_ME = graphql(`
  query Me {
    me {
      id
      email
      name
      teams {
        id
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
