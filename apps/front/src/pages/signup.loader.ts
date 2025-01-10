import { LoaderFunctionArgs } from 'react-router'
import { apolloClient } from '@/providers/apollo.js'
import { graphql } from '@/__generated__/gql.js'
import { InvitationTokenQuery } from '@/__generated__/graphql.js'

const GET_INVITATION_TOKEN = graphql(`
  query InvitationToken($token: String!) {
    invitation(token: $token) {
      id
      expiresAt
      token
      email
    }
  }
`)

type Params = {
  invitationToken: string
}

export async function signupLoader({
  params: { invitationToken },
}: LoaderFunctionArgs<Params>) {
  const { error, data } = await apolloClient.query<InvitationTokenQuery>({
    query: GET_INVITATION_TOKEN,
    variables: {
      token: invitationToken,
    },
  })
  if (error) throw error
  return data
}
