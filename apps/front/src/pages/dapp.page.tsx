import { gql, useQuery, useSubscription } from '@apollo/client'
import { useParams } from 'react-router'
import { Box, Heading, Skeleton, Stack } from '@chakra-ui/react'
import { graphql } from '@/__generated__/gql.js'

import {
  DappUpdatedSubscription,
  GetDappDetailsQuery,
} from '@/__generated__/graphql.js'

import { DappStatus } from '@/components/dapp-status/dapp-status.js'
import { BlockSimple } from '@/components/stats-blocks/block-simple'

const GET_DAPP_DETAILS = graphql(`
  query GetDappDetails($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
      stats {
        id
        name
        timestamp
        externalRef
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

  if (error) {
    throw Error(error.message)
  }

  return (
    <Box>
      {data ? (
        <Stack direction="row" align="center">
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
            <BlockSimple
              title="Total FHE Events"
              amount={data?.dapp.stats.length || 0}
            />
          </Stack>
        </Stack>
      )}
    </Box>
  )
}
