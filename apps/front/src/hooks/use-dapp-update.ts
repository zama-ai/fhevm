import { UpdateDappMutation } from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback } from 'react'

export function useDappUpdate(dappId: string) {
  const [updateDapp, { data, loading, error }] = useMutation<
    UpdateDappMutation,
    { dappId: string; name?: string; address?: string }
  >(UPDATE_DAPP)

  const handleUpdateDapp = useCallback(
    (variables: { name: string; description?: string }) =>
      updateDapp({
        variables: {
          ...variables,
          dappId,
        },
      }),
    [dappId, updateDapp],
  )

  return {
    updateDapp: handleUpdateDapp,
    loading,
    error,
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
