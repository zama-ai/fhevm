import { CreateApiKeyMutation } from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'
import { useCallback } from 'react'

export function useCreateApiKey(dappId: string) {
  const [createApiKey, { data, loading, error }] = useMutation<
    CreateApiKeyMutation,
    { dappId: string; name: string; description?: string }
  >(CREATE_API_KEY, {
    // TODO: invalidate the list API keys query
    refetchQueries: [],
  })

  const handleCreateApiKey = useCallback(
    async (variables: { name: string; description?: string }) => {
      try {
        // const response = await createApiKey({
        await createApiKey({
          variables: {
            ...variables,
            dappId,
          },
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
    loading,
    error,
    token: data?.createApiKey.token,
    apiKeyId: data?.createApiKey.apiKey.id,
  }
}

const CREATE_API_KEY = gql(`
  mutation CreateApiKey(
    $dappId: String!
    $name: String!
    $description: String
  ) {
    createApiKey(
      input: { dappId: $dappId, name: $name, description: $description }
    ) {
      token
      apiKey {
        id
        dappId
        name
        description
      }
    }
  }
`)
