import { memo } from 'react'
import { Outlet, useLoaderData } from 'react-router'

import { Box, Flex } from '@chakra-ui/react'
import { Header } from '@/components/header/header'
import { Navigation } from '@/components/navigation/navigation'
import { MeQuery } from '@/__generated__/graphql'
import { getPersonalTeam } from '@/lib/personal-team'

const HeaderMemo = memo(Header)

export function PrivateLayout() {
  const { me } = useLoaderData<MeQuery>()
  const dapps = me && getPersonalTeam(me.teams).dapps
  return (
    <Box className="layout-private">
      <HeaderMemo name={me.name} />
      <Flex direction="row" wrap="nowrap" justify="flex-start" align="stretch">
        <Box display={{ base: 'none', lg: 'block' }}>
          {dapps && <Navigation dapps={dapps} />}
        </Box>
        <Box p="40px" flexGrow="1">
          <Outlet />
        </Box>
      </Flex>
    </Box>
  )
}
