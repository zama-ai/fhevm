import { Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'

import { gql, useSubscription } from '@apollo/client'
import { DummyLiveSubscription } from '@/__generated__/graphql'

const GET_PROJECT_LIVE = gql(`
  subscription DummyLive {
    dummy {
      id
      name
    }
  }
`)

export function DefaultPage() {
  const { data: liveData } =
    useSubscription<DummyLiveSubscription>(GET_PROJECT_LIVE)
  return (
    <Stack gap="4">
      <Text>
        <Link to="/signin">signin</Link>
        {liveData?.dummy?.id}
      </Text>
    </Stack>
  )
}
