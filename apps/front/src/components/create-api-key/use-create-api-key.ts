import { CreateApiKeyMutation } from '@/__generated__/graphql'
import { gql, useMutation } from '@apollo/client'

export function useCreateApiKey() {
  const [createApiKey, { data, loading, error }] = useMutation<
    CreateApiKeyMutation,
    { dappId: string; name: string; description?: string }
  >(CREATE_API_KEY, {
    // TODO: invalidate the list API keys query
    refetchQueries: ['ListApiKeys'],
  })

  return {
    createApiKey,
    loading,
    errorMessage: error?.message,
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
