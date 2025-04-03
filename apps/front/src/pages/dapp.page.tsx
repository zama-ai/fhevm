import { useCallback } from 'react'
import { gql, useMutation, useQuery, useSubscription } from '@apollo/client'
import { useParams } from 'react-router'
import { Box, Heading, Skeleton, Stack } from '@chakra-ui/react'
import { graphql } from '@/__generated__/gql.js'

import {
  DappUpdatedSubscription,
  GetDappDetailsQuery,
  CreateApiKeyMutation,
} from '@/__generated__/graphql.js'

import { DappStatus } from '@/components/dapp-status/dapp-status.js'
import { BlockSimple } from '@/components/stats-blocks/block-simple'
import { BlockPie } from '@/components/stats-blocks/block-pie'
import {
  calculateOperationStats,
  calculateEncryptionStats,
  calculateTotal,
} from '@/lib/stats.js'
import { CreateApiKey } from '@/components/create-api-key/create-api-key'

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

const GET_DAPP_DETAILS = graphql(`
  query GetDappDetails($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
      rawStats {
        id
        name
        timestamp
        externalRef
      }
      stats {
        id
        cumulative {
          total
          FheAdd
          FheBitAnd
          FheIfThenElse
          FheLe
          FheOr
          FheSub
          TrivialEncrypt
          VerifyCiphertext
          FheMul
          FheDiv
        }
      }
    }
  }
`)

const SUB_DAPP_UPDATED = gql(`
  subscription DappUpdated($dappId: ID!) {
    dappUpdated(input: { id: $dappId }) {
      id
      name
      status
      rawStats {
        id
        name
        timestamp
        externalRef
      }
      stats {
        id
        cumulative {
          total
        }
      }
    } 
  }
`)

export function DappPage() {
  const { dappId } = useParams()
  const { data, error } = useQuery<GetDappDetailsQuery>(GET_DAPP_DETAILS, {
    variables: { dappId },
  })
  const { data: liveData } = useSubscription<DappUpdatedSubscription>(
    SUB_DAPP_UPDATED,
    {
      variables: { dappId },
    },
  )

  const [createApiKeyMutation, { error: createApiError }] =
    useMutation<CreateApiKeyMutation>(CREATE_API_KEY, {})

  if (error) {
    throw Error(error.message)
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { __typename, total, ...cumulative } = data?.dapp.stats.cumulative || {
    total: 0,
  }
  const operationStatsData = calculateOperationStats(cumulative)
  const operationStatsTotal = calculateTotal(operationStatsData)
  const encryptionStatsData = calculateEncryptionStats(cumulative)
  const encryptionStatsTotal = calculateTotal(encryptionStatsData)

  const handleApiKeyOnCreate = useCallback(
    ({ name, description }: { name: string; description?: string }) => {
      createApiKeyMutation({
        variables: {
          name,
          description,
          dappId,
        },
      })
    },
    [dappId, createApiKeyMutation],
  )

  return (
    <Box>
      {data ? (
        <Stack direction="row" align="center" alignItems="flex-start">
          <Heading mb="5">
            {liveData ? liveData.dappUpdated.name : data.dapp.name}
          </Heading>
          <DappStatus
            status={liveData ? liveData.dappUpdated.status : data.dapp.status}
            ml="2"
            size="xs"
          />
        </Stack>
      ) : (
        <Skeleton height="5" my="5" width="30rem" />
      )}
      {data && (
        <Stack direction="column" gap="5">
          <Stack direction="row" gap="5">
            <BlockSimple title="Total FHE Events" amount={total} />
            <BlockPie
              title="FHE Operations"
              total={operationStatsTotal || 0}
              data={operationStatsData}
            />
            <BlockPie
              title="FHE Encryption"
              total={encryptionStatsTotal || 0}
              data={encryptionStatsData}
            />
          </Stack>
        </Stack>
      )}
      {dappId && (
        <CreateApiKey error={createApiError} onCreate={handleApiKeyOnCreate} />
      )}
    </Box>
  )
}
