import { apolloClient } from '@/providers/apollo'
import { graphql } from '@/__generated__/gql'
import { MeTeamDappsQuery } from '@/__generated__/graphql'

// load and cache general informations for faster rendering
const GET_ME_TEAMS_DAPPS = graphql(`
  query MeTeamDapps {
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

export async function dashboardLoader() {
  const { error, data } = await apolloClient.query<MeTeamDappsQuery>({
    query: GET_ME_TEAMS_DAPPS,
  })
  if (error) throw error
  return data
}
