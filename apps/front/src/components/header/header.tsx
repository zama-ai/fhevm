import {
  Box,
  Flex,
  Text,
  IconButton,
  Stack,
  HStack,
  Collapsible,
  useBreakpointValue,
  useDisclosure,
} from '@chakra-ui/react'

import { useColorModeValue } from '@/hooks/use-color-mode'
import { CloseIcon, HamburgerIcon } from '@/components/icons/icons'
import { Logo } from '@/components/logo/logo'
import { MeMenu } from '@/components/me-menu/me-menu'
import { Link } from '@/components/ui/link'

type OwnProps = {
  name: string
}

export function Header({ name }: OwnProps) {
  const { open, onToggle } = useDisclosure()
  return (
    <Box>
      <Flex bg="brand" color="black" h="70px" py="2" px="4" align="center">
        <Flex
          flex={{ base: 1, md: 'auto' }}
          ml={-2}
          display={{ base: 'flex', md: 'none' }}
          alignItems="center"
        >
          <IconButton
            onClick={onToggle}
            variant="ghost"
            aria-label="Toggle Navigation"
            color="black"
          >
            {open ? (
              <CloseIcon boxSize="25px" />
            ) : (
              <HamburgerIcon boxSize="25px" />
            )}
          </IconButton>
        </Flex>
        <Flex flex={1} justify={{ base: 'center', md: 'start' }}>
          <Link to="/">
            <Logo width={100} />
          </Link>
        </Flex>

        <HStack flex={1} justify={'flex-end'} direction={'row'}>
          <Flex display={{ base: 'none', md: 'flex' }} ml={10}>
            <DesktopNav />
          </Flex>
          <MeMenu
            condensed={useBreakpointValue({ base: true, md: false })}
            name={name}
            email="janesilas@web3.dev"
            image="https://cdn.prod.website-files.com/6471ebc32c5012b32f0e45ba/66bc763b1cd88de111ad0182_zygLzKmbHcXkBxyHrt47tBrwTi3ZBBwp86Qe8gI11bs.png"
          />
        </HStack>
      </Flex>
      <Collapsible.Root open={open}>
        <Collapsible.Content>
          <MobileNav />
        </Collapsible.Content>
      </Collapsible.Root>
    </Box>
  )
}

const DesktopNav = () => {
  return (
    <Stack direction={'row'}>
      <Text fontWeight="bold">Need Help?</Text>
    </Stack>
  )
}

const MobileNav = () => {
  const bg = useColorModeValue('white', 'gray.800')
  return (
    <Stack bg={bg} p={4} display={{ md: 'none' }}>
      <Text>Need Help?</Text>
    </Stack>
  )
}
