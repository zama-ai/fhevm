import { useContext } from 'react'
import { Box, List, Stack } from '@chakra-ui/react'
import { useLocation } from 'react-router'

import {
  DocumentationIcon,
  DashboardIcon,
  LearnIcon,
  CommunityIcon,
} from '@/components/icons/icons'

import { TitleContext } from '@/components/title-context/title-context'
import { NavBlock } from './nav-block'
import { NavApp, NewNavApp } from './nav-app'

export function Navigation() {
  const { pathname } = useLocation()
  const { title } = useContext(TitleContext)
  return (
    <Box
      className="navigation"
      flexBasis="300px"
      flexGrow="0"
      flexShrink="0"
      borderRightStyle="solid"
      borderRightWidth="1px"
      borderRightColor="gray.200"
      px="50px"
      py="30px"
    >
      <Stack>
        <NavBlock title="Dashboard" icon={DashboardIcon} to="/dashboard" />
        <List.Root gap="2" variant="plain" align="center">
          {pathname === '/create' && <NewNavApp name={title} />}
          <NavApp name="My Hello World dApp" status="active" />
          <NavApp name="My other dApp" status="draft" />
        </List.Root>
        <NavBlock title="Documentation" icon={DocumentationIcon} to="/about" />
        <NavBlock title="Learn" icon={LearnIcon} to="/" />
        <NavBlock
          title="Community"
          icon={CommunityIcon}
          to="https://zama.ai/community"
        />
      </Stack>
    </Box>
  )
}
