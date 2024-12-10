import { useContext } from 'react'
import { Box, List, Stack, Text, HStack } from '@chakra-ui/react'
import { Circle } from 'lucide-react'
import {
  DocumentationIcon,
  DashboardIcon,
  LearnIcon,
  CommunityIcon,
} from '@/components/icons/icons'

import { TitleContext } from '@/components/title-context/title-context'
import { NavLink } from '../ui/link'

type NavBlockProps = {
  title: string
  icon: typeof DocumentationIcon
  to: string
}

function NavBlock({ title, icon, to }: NavBlockProps) {
  return (
    <NavLink to={to}>
      {({ isActive }) => (
        <HStack
          bg={isActive ? 'ivory' : 'transparent'}
          width="100%"
          p="2"
          rounded="md"
        >
          <Box as={icon} />
          <Text fontWeight={isActive ? 'bold' : 'normal'}>{title}</Text>
        </HStack>
      )}
    </NavLink>
  )
}

type NavAppProps = {
  name: string
  status: string
}

function NavApp({ name, status }: NavAppProps) {
  const color = status === 'active' ? 'green.200' : 'gray.300'
  return (
    <List.Item>
      <NavLink to="/app/1" className="group">
        <List.Indicator
          asChild
          color={color}
          width="10px"
          opacity={0}
          _groupHover={{ opacity: 1 }}
          transition="opacity .5s"
        >
          <Circle className="circle" />
        </List.Indicator>
        <Text fontSize="sm">{name}</Text>
      </NavLink>
    </List.Item>
  )
}

export function Navigation() {
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
        <List.Root pl="2" gap="2" variant="plain" align="center">
          <NavApp name="My Hello World dApp" status="active" />
          <NavApp name="My other dApp" status="draft" />
          <NavApp name={title} status="draft" />
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
