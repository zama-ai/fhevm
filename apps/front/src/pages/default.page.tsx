import { Box, Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'

import { gql, useSubscription } from '@apollo/client'
import {
  DappUpdatedSubscription,
  DummyLiveSubscription,
} from '@/__generated__/graphql'

const GET_PROJECT_LIVE = gql(`
  subscription DummyLive($id: ID!) {
    dummy(input: { id: $id}) {
      id
      name
    }
  }
`)

const SUB_DAPP_UPDATED = gql(`
  subscription DappUpdated($id: ID!) {
    dappUpdated(input: { id: $id }) {
      id
      name
      status
    }
  }
`)

export function DefaultPage() {
  const { data: dummyData } = useSubscription<DummyLiveSubscription>(
    GET_PROJECT_LIVE,
    {
      variables: { id: 'dapp_123abc' },
    },
  )
  const { data: dappData } = useSubscription<DappUpdatedSubscription>(
    SUB_DAPP_UPDATED,
    {
      variables: { id: 'dapp_cRcSlh0_the9' },
    },
  )
  return (
    <Stack gap="4">
      <Text>
        <Link to="/signin">signin</Link>
      </Text>
      <Box>{localStorage.getItem('token') ? 'has token' : 'no token'}</Box>
      <Box>dummy:{dummyData?.dummy?.id}</Box>
      <Box>
        dapp:{dappData?.dappUpdated?.id} / {dappData?.dappUpdated?.name} /{' '}
        {dappData?.dappUpdated?.status}
      </Box>
    </Stack>
  )
}
