import { memo } from 'react'
import { Outlet } from 'react-router'

import { Box, Flex } from '@chakra-ui/react'
import { Header } from '@/components/header/header.js'
import { Navigation } from '@/components/navigation/navigation.js'
import { PrivateLayoutSkeleton } from '@/layouts/private.layout.skeleton.js'
import { MeQuery } from '@/__generated__/graphql.js'
import { getPersonalTeam } from '@/lib/personal-team.js'
import { GET_ME } from '@/queries'
import { useQuery } from '@apollo/client'

const HeaderMemo = memo(Header)

export function PrivateLayout() {
  const { loading, error, data } = useQuery<MeQuery>(GET_ME)
  if (error) throw new Error(error.message)
  if (loading || !data) return <PrivateLayoutSkeleton />

  const me = data?.me || null
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
