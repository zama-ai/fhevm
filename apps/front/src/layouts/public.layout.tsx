import { useLocation } from 'react-router'
import { Outlet } from 'react-router'
import { Box, Flex, Stack, Text } from '@chakra-ui/react'
import { Logo } from '@/components/logo/logo.js'
import { Link } from '@/components/ui/link.js'

function Help() {
  return (
    <Text color={{ base: 'gray.500', md: 'yellow.700' }} textStyle="sm">
      Console is in closed beta. Please{' '}
      <Link to="https://zama.ai">contact us</Link> for details.
    </Text>
  )
}

function ContextualPlaceholder() {
  const { pathname } = useLocation()

  if (pathname === '/signin')
    return (
      <>
        <img
          src="https://cdn.prod.website-files.com/61bc21e3a843412266a08eb3/66e06d589847b94a486638a3_hero%20images%20(8).png"
          alt="Sign in illustration of a city in the Eth realm"
          style={{ width: '75%' }}
        />
      </>
    )
  if (/^\/signup/.test(pathname))
    return (
      <video autoPlay={true} loop muted playsInline style={{ width: '50%' }}>
        <source
          src="https://s3.amazonaws.com/webflow-prod-assets/61bc21e3a843412266a08eb3/65afbf275af8444b89634c90_Zama-Encrypted-alpha-hevc-safari%20(1).mp4"
          type='video/mp4; codecs="hvc1"'
        />
        <source
          src="https://s3.amazonaws.com/webflow-prod-assets/61bc21e3a843412266a08eb3/65afbdd9ff0650d791d64941_Zama-Encrypted-alpha-vp9-chrome%20(1).webm"
          type="video/webm"
        />
      </video>
    )

  // default: return an enpty space
  return <></>
}

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
        bg="brand.500"
      >
        <Flex minH="70px" py="2" px="4" bg="brand.500" alignItems="center">
          <Link to="/">
            <Logo width={100} />
          </Link>
        </Flex>
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
            <ContextualPlaceholder />
          </Box>
        </Flex>
        <Box p="4" display={{ base: 'none', md: 'block' }}>
          <Help />
        </Box>
      </Box>
      <Box
        flexBasis="50%"
        display="flex"
        alignItems="center"
        p={8}
        flexShrink={1}
        flexGrow={1}
      >
        <Stack flexShrink={1} flexGrow={1}>
          <Outlet />
          <Box py="4" display={{ base: 'block', md: 'none' }}>
            <Help />
          </Box>
        </Stack>
      </Box>
    </Flex>
  )
}
