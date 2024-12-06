import { apolloClient } from '@/providers/apollo'
import { graphql } from '../__generated__/gql'

const GET_ME = graphql(`
  query Me {
    me {
      id
      email
      name
    }
  }
`)

export async function privateLoader() {
  const { error, data } = await apolloClient.query({
    query: GET_ME,
  })
  console.log('privateLoader', { error, data })
  if (error) {
    throw error
  }

  return data
}
