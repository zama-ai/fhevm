import { Stack } from '@chakra-ui/react'
import { useApiKeys } from './use-api-keys'
import { ApiKeyCard, SkeletonApiKeyCard } from './api-key-card'
import { useDeleteApiKey } from './use-delete-api-key'

export type ListApiKeysProps = {
  dappId: string
}

export function ListApiKeys({ dappId }: ListApiKeysProps) {
  const { apiKeys, loading } = useApiKeys(dappId)
  const { deleteApiKey } = useDeleteApiKey()

  return (
    <Stack role="list">
      {loading ? (
        <>
          <SkeletonApiKeyCard />
          <SkeletonApiKeyCard />
          <SkeletonApiKeyCard />
        </>
      ) : (
        apiKeys?.dapp.apiKeys.map(apiKey => (
          <ApiKeyCard
            key={apiKey.id}
            id={apiKey.id}
            name={apiKey.name}
            description={apiKey.description ?? null}
            onDelete={deleteApiKey}
          />
        ))
      )}
    </Stack>
  )
}
