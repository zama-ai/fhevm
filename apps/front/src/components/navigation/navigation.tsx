import { useContext } from 'react'
import { Box, List, Stack } from '@chakra-ui/react'
import { useLocation } from 'react-router'

import {
  DocumentationIcon,
  DashboardIcon,
  LearnIcon,
  CommunityIcon,
} from '@/components/icons/icons.js'

import { TitleContext } from '@/components/title-context/title-context.js'
import { NavBlock } from './nav-block.js'
import { NavApp, NewNavApp } from './nav-app.js'

type OwnProps = {
  dapps: Array<{
    id: string
    name: string
    status: string
  }>
}

export function Navigation({ dapps }: OwnProps) {
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
          {/^\/create\/team_/.test(pathname) && <NewNavApp name={title} />}
          {dapps.map(dapp => (
            <NavApp
              key={dapp.id}
              id={dapp.id}
              name={dapp.name}
              status={dapp.status}
            />
          ))}
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
