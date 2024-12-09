import { apolloClient } from '@/providers/apollo'
import { graphql } from '../__generated__/gql'

// load and cache general informations for faster rendering
const GET_ME = graphql(`
  query Me {
    me {
      id
      email
      name
    }
  }
`)

export async function dashboardLoader() {
  const { error, data } = await apolloClient.query({
    query: GET_ME,
  })
  if (error) throw error
  return data
}
