import { memo } from 'react'
import { Outlet } from 'react-router'

import { Box, Flex } from '@chakra-ui/react'
import { Header } from '@/components/header/header'
import { Navigation } from '@/components/navigation/navigation'

const HeaderMemo = memo(Header)

export function PrivateLayout() {
  return (
    <Box className="layout-private">
      <HeaderMemo />
      <Flex direction="row" wrap="nowrap" justify="flex-start" align="stretch">
        <Navigation />
        <Box p="40px" flexGrow="1">
          <Outlet />
        </Box>
      </Flex>
    </Box>
  )
}
