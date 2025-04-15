import {
  ListApiKeysQuery,
  ListApiKeysQueryVariables,
} from '@/__generated__/graphql'
import { gql, useQuery } from '@apollo/client'

export function useApiKeys(dappId: string) {
  const { data, loading, error } = useQuery<
    ListApiKeysQuery,
    ListApiKeysQueryVariables
  >(LIST_API_KEYS, {
    variables: { dappId },
  })

  return {
    apiKeys: data,
    loading,
    error: error?.message,
  }
}

const LIST_API_KEYS = gql(`
  query ListApiKeys($dappId: ID!) {
    dapp(input: {
      id: $dappId
    }) {
      apiKeys {
        id
        name
        description
        createdAt
      }
    }
  }
`)
