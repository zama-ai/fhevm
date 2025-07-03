import { LoaderFunctionArgs } from 'react-router'
import { apolloClient } from '@/providers/apollo.js'
import { graphql } from '@/__generated__/gql.js'
import { ValidationTokenMutation } from '@/__generated__/graphql.js'

const GET_VALIDATION_TOKEN = graphql(`
  mutation ValidationToken($token: String!) {
    confirmEmail(input: { token: $token }) {
      token
    }
  }
`)

type Params = {
  invitationToken: string
}

export async function validationLoader({
  params: { validationToken },
}: LoaderFunctionArgs<Params>) {
  console.log({ validationToken })
  const { data } = await apolloClient.mutate<ValidationTokenMutation>({
    mutation: GET_VALIDATION_TOKEN,
    variables: {
      token: validationToken,
    },
    fetchPolicy: 'network-only',
  })
  if (!data) throw new Error('No data returned from validation mutation')
  return data
}
