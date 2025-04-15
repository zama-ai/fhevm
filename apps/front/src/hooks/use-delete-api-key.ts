import {
  DeleteApiKeyMutation,
  DeleteApiKeyMutationVariables,
} from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback } from 'react'

/**
 * Delete an API Key by its ID.
 *
 * @param apiKeyId
 */
export function useDeleteApiKey() {
  const [deleteApiKey] = useMutation<
    DeleteApiKeyMutation,
    DeleteApiKeyMutationVariables
  >(DELETE_API_KEY, {
    // TODO: invalidate the list API key query
    refetchQueries: [],
  })

  const handleDeleteApiKey = useCallback(
    (apiKeyId: string) => {
      deleteApiKey({
        variables: {
          apiKeyId,
        },
        refetchQueries: ['ListApiKeys'],
      })
    },
    [deleteApiKey],
  )

  return {
    deleteApiKey: handleDeleteApiKey,
  }
}

const DELETE_API_KEY = gql(`
  mutation DeleteApiKey($apiKeyId: ID!) {
    deleteApiKey(input: {
      id: $apiKeyId
    })
  }
`)
