import { apolloClient } from '@/providers/apollo'
import { graphql } from '../__generated__/gql'

// load and cache general informations for faster rendering
const GET_ME = graphql(`
  query Me {
    me {
      id
      email
    }
  }
`)

export async function dashboardLoader() {
  const { data } = await apolloClient.query({
    query: GET_ME,
  })
  return data
}
