import { graphql } from '@/__generated__/gql.js'

// Queries that need to be reused across multiple components or refetched

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

export const GET_ME_TEAMS_DAPPS = graphql(`
  query MeTeamsDapps {
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
