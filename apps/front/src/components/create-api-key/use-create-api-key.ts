import { graphql } from '@/__generated__/gql'
import { CreateApiKeyMutation } from '@/__generated__/graphql'
import { useMutation } from '@apollo/client'
import { useCallback } from 'react'

export function useCreateApiKey(dappId: string) {
  const [createApiKey, { data, loading, error }] = useMutation<
    CreateApiKeyMutation,
    { dappId: string; name: string; description?: string }
  >(CREATE_API_KEY)

  const handleCreateApiKey = useCallback(
    async (variables: { name: string; description?: string }) => {
      try {
        // const response = await createApiKey({
        await createApiKey({
          variables: {
            ...variables,
            dappId,
          },
          // TODO: invalidate the list API keys query
          refetchQueries: [],
        })
        // return response.data
      } catch (error) {
        // Handle or rethrow the error if needed
        console.error(`Failed to create API key: ${error}`)
        // throw error
      }
    },
    [dappId, createApiKey],
  )

  return {
    createApiKey: handleCreateApiKey,
    data,
    loading,
    error,
  }
}

const CREATE_API_KEY = graphql(`
  mutation createApiKey(
    $dappId: String!
    $name: String!
    $description: String
  ) {
    createApiKey(
      input: { dappId: $dappId, name: $name, description: $description }
    ) {
      id
      dappId
      name
      description
    }
  }
`)
