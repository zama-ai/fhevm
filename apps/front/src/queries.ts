import { graphql } from '@/__generated__/gql.js'
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
