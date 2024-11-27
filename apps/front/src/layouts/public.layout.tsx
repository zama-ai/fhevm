import { NavLink } from 'react-router'
import { Outlet } from 'react-router'
import { Box, Flex, Heading } from '@chakra-ui/react'
import { Logo } from '@/components/logo/logo'

export function PublicLayout() {
  return (
    <Flex
      minWidth="100vw"
      minHeight={{ base: '60px', md: '100vh' }}
      flexDirection={{ base: 'column', md: 'row' }}
    >
      <Box
        flexBasis="50%"
        alignItems="center"
        justifyContent="center"
        flexShrink={1}
        flexGrow={1}
        bg="brand"
      >
        <Box minH={'60px'} py={{ base: 2 }} px={{ base: 4 }} bg="brand">
          <NavLink to="/">
            <Logo width={100} />
          </NavLink>
        </Box>
        <Flex
          minHeight="calc(100vh - 120px)"
          display={{ base: 'none', md: 'flex' }}
        >
          <Box
            flex="1"
            display="flex"
            alignItems="center"
            justifyContent="center"
          >
            <Heading>Welcome back!</Heading>
          </Box>
        </Flex>
      </Box>
      <Box
        flexBasis="50%"
        display="flex"
        alignItems="center"
        p={8}
        flexShrink={1}
        flexGrow={1}
      >
        <Outlet />
      </Box>
    </Flex>
  )
}
