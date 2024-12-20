import { useQuery } from '@apollo/client'
import { useParams } from 'react-router'
import { Box, Heading, Skeleton, Stack } from '@chakra-ui/react'
import { graphql } from '@/__generated__/gql'
import { GetDappQuery } from '@/__generated__/graphql'
import { BlockUsageChart } from '@/components/stats-blocks/block-usage-chart'
import { DappStatus } from '@/components/dapp-status/dapp-status'
import { BlockUaw } from '@/components/stats-blocks/block-uaw'

const GET_DAPP = graphql(`
  query GetDapp($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
    }
  }
`)

export function DappPage() {
  const { dappId } = useParams()
  const { data, error } = useQuery<GetDappQuery>(GET_DAPP, {
    variables: { dappId },
  })

  if (error) {
    throw Error(error.message)
  }

  return (
    <Box>
      {data ? (
        <Stack direction="row" align="center">
          <Heading my="5">{data.dapp.name}</Heading>
          <DappStatus status={data.dapp.status} ml="2" size="xs" />
        </Stack>
      ) : (
        <Skeleton height="5" my="5" width="30rem" />
      )}

      <Stack direction="row" gap="5">
        <BlockUsageChart />
        <BlockUaw
          title="Unique Active Wallets"
          amount={182}
          percentage={12}
          description="Since last month"
        />
      </Stack>
    </Box>
  )
}
