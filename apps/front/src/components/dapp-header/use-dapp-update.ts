import { UpdateDappMutation } from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'

export function useDappUpdate() {
  const [updateDapp, { data, loading, error }] = useMutation<
    UpdateDappMutation,
    { dappId: string; name: string; address?: string }
  >(UPDATE_DAPP)

  return {
    updateDapp,
    loading,
    errorMessage: error?.message,
    updatedDapp: data,
  }
}

export const UPDATE_DAPP = gql(`
  mutation UpdateDapp(
    $dappId: ID!
    $name: String
    $address: String
  ) {
    updateDapp(input: { id: $dappId, name: $name, address: $address }) {
      id
      name
      status
      address
      team {
        id
        name
      }
    }
  }
`)
