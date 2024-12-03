import { Box, Group, Text } from '@chakra-ui/react'
import { Settings, LogOut, ChevronDown } from 'lucide-react'
import { MenuContent, MenuItem, MenuRoot, MenuTrigger } from '../ui/menu'
import { Avatar } from '../ui/avatar'

type MeMenuProps = {
  name: string
  email: string
  image: string
  condensed?: boolean
}

export function MeMenu({ name, image, condensed }: MeMenuProps) {
  return (
    <MenuRoot positioning={{ placement: 'bottom-end' }}>
      <MenuTrigger asChild>
        <Group
          gap="3"
          rounded={'md'}
          _hover={{ bg: '#f9dc5c' }}
          p={2}
          style={{ cursor: 'pointer' }}
        >
          <Avatar size="sm" src={image} name={name} />
          <Text
            display={condensed ? 'none' : 'inline-block'}
            maxW={120}
            overflow="hidden"
            whiteSpace="nowrap"
            textOverflow="ellipsis"
          >
            {name}
          </Text>
          <ChevronDown />
        </Group>
      </MenuTrigger>
      <MenuContent>
        <MenuItem value="preferences">
          <Settings />
          <Box flex="1">Preferences</Box>
        </MenuItem>
        <MenuItem value="logout">
          <LogOut />
          <Box flex="1">Sign out</Box>
        </MenuItem>
      </MenuContent>
    </MenuRoot>
  )
}
