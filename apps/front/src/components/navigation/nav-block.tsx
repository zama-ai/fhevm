import { Box, Text, HStack } from '@chakra-ui/react'
import { DocumentationIcon } from '@/components/icons/icons.js'
import { NavLink } from '@/components/ui/link.js'

type OwnProps = {
  title: string
  icon: typeof DocumentationIcon
  to: string
}

export function NavBlock({ title, icon, to }: OwnProps) {
  return (
    <NavLink to={to}>
      {({ isActive }) => (
        <HStack
          bg={isActive ? 'brand.subtle' : 'transparent'}
          _dark={{ bg: isActive ? 'brand.subtle' : 'transparent' }}
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
