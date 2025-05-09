import { ListChainsQuery } from '@/__generated__/graphql'
import { gql, useQuery } from '@apollo/client'

const LIST_CHAINS = gql(`
  query ListChains {
    chains {
      id
      name
      description
    }
  }
`)

export function useChains() {
  const { data, loading, error } = useQuery<ListChainsQuery>(LIST_CHAINS, {
    variables: {},
  })
  return {
    chains: data?.chains ?? [],
    loading,
    error: error?.message,
  }
}
