import { Box, EmptyState, Table } from '@chakra-ui/react'
import { Code, KeyRound, Skull } from 'lucide-react'
import { useApiKeys } from '@/components/list-api-keys/use-api-keys'
import { useDeleteApiKey } from '@/components/list-api-keys/use-delete-api-key'
import { ApiKeyItem, SkeletonApiKeyItem } from './api-key-item'

export type ListApiKeysProps = {
  dappId: string
}

export function ListApiKeys({ dappId }: ListApiKeysProps) {
  const { apiKeys, loading, errorMessage } = useApiKeys(dappId)
  const { deleteApiKey } = useDeleteApiKey()

  return (
    <Box maxW="2xl">
      <Table.Root size="sm" variant="outline" rounded="md">
        {errorMessage && (
          <EmptyState.Root>
            <EmptyState.Content>
              <EmptyState.Indicator>
                <Skull />
              </EmptyState.Indicator>
              <EmptyState.Title>Error</EmptyState.Title>
              <EmptyState.Description>
                There was an error while trying to fetch the API keys. Please
                contact support
                <Code>{errorMessage}</Code>
              </EmptyState.Description>
            </EmptyState.Content>
          </EmptyState.Root>
        )}
        {apiKeys?.dapp.apiKeys.length === 0 ? (
          <EmptyState.Root>
            <EmptyState.Content>
              <EmptyState.Indicator>
                <KeyRound />
              </EmptyState.Indicator>
              <EmptyState.Title>No API keys</EmptyState.Title>
              <EmptyState.Description>
                Create an API key to enable private descriptions.
              </EmptyState.Description>
            </EmptyState.Content>
          </EmptyState.Root>
        ) : (
          <>
            <Table.Header>
              <Table.Row>
                <Table.ColumnHeader>Name</Table.ColumnHeader>
                <Table.ColumnHeader>Created</Table.ColumnHeader>
                <Table.ColumnHeader></Table.ColumnHeader>
              </Table.Row>
            </Table.Header>
            <Table.Body role="list">
              {loading && (
                <>
                  <SkeletonApiKeyItem key="a" />
                  <SkeletonApiKeyItem key="b" />
                </>
              )}

              {apiKeys &&
                [...apiKeys.dapp.apiKeys]
                  .sort((a, b) => b.createdAt - a.createdAt)
                  .map(item => (
                    <ApiKeyItem
                      key={item.id}
                      id={item.id}
                      name={item.name}
                      description={item.description ?? null}
                      createdAt={item.createdAt}
                      onDelete={deleteApiKey}
                    />
                  ))}
            </Table.Body>
          </>
        )}
      </Table.Root>
    </Box>
  )
}
