import { useCallback } from 'react'
import { useNavigate } from 'react-router'
import { Box, Group, Text } from '@chakra-ui/react'

import {
  MenuContent,
  MenuItem,
  MenuRoot,
  MenuTrigger,
} from '#components/ui/menu.js'
import { Avatar } from '#components/ui/avatar.js'
import {
  ChevronDownIcon,
  LogOutIcon,
  SettingsIcon,
} from '#components/icons/icons.js'

type MeMenuProps = {
  name: string
  email: string
  image: string
  condensed?: boolean
}

export function MeMenu({ name, image, condensed }: MeMenuProps) {
  const navigate = useNavigate()
  const onLogout = useCallback(() => {
    localStorage.removeItem('token')
    navigate('/signin')
  }, [navigate])
  return (
    <MenuRoot positioning={{ placement: 'bottom-end' }}>
      <MenuTrigger asChild>
        <Group
          gap="3"
          rounded={'md'}
          _hover={{ bg: 'brand.emphasized' }}
          p={2}
          style={{ cursor: 'pointer' }}
        >
          <Avatar size="sm" src={image} name={name} />
          {!condensed && (
            <Text
              maxW={120}
              overflow="hidden"
              whiteSpace="nowrap"
              textOverflow="ellipsis"
            >
              {name}
            </Text>
          )}
          <ChevronDownIcon />
        </Group>
      </MenuTrigger>
      <MenuContent>
        <MenuItem value="preferences">
          <SettingsIcon />
          <Box flex="1">Preferences</Box>
        </MenuItem>
        <MenuItem value="logout" onClick={onLogout}>
          <LogOutIcon />
          <Box flex="1">Sign out</Box>
        </MenuItem>
      </MenuContent>
    </MenuRoot>
  )
}
