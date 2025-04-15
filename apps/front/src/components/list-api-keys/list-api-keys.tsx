import { Box, EmptyState, Table } from '@chakra-ui/react'
import { useApiKeys } from './use-api-keys'
import { KeyRound } from 'lucide-react'
import { ApiKeyItem, SkeletonApiKeyItem } from './api-key-item'
import { useDeleteApiKey } from './use-delete-api-key'

export type ListApiKeysProps = {
  dappId: string
}

export function ListApiKeys({ dappId }: ListApiKeysProps) {
  const { apiKeys, loading } = useApiKeys(dappId)
  const { deleteApiKey } = useDeleteApiKey()

  return (
    <Box maxW="2xl">
      <Table.Root size="sm" interactive variant="outline" rounded="md">
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
