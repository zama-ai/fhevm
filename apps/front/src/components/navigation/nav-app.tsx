import { List, Text } from '@chakra-ui/react'
import { Circle } from 'lucide-react'
import { NavLink } from '@/components/ui/link'

type NavAppProps = {
  name: string
  status: string
}

export function NavApp({ name, status }: NavAppProps) {
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
        <Text
          fontSize="sm"
          overflow="hidden"
          textOverflow="ellipsis"
          textWrap="nowrap"
          maxWidth="130px"
        >
          {name}
        </Text>
      </NavLink>
    </List.Item>
  )
}

type CurrentNavAppProps = {
  name: string
}

export function NewNavApp({ name }: CurrentNavAppProps) {
  return (
    <List.Item
      className="group"
      bg="brand.subtle"
      _dark={{ bg: 'brand.subtle' }}
      rounded="md"
      pl="2"
    >
      <List.Indicator
        asChild
        color="gray.300"
        width="10px"
        opacity={0}
        _groupHover={{ opacity: 1 }}
        transition="opacity .5s"
      >
        <Circle className="circle" />
      </List.Indicator>
      <Text
        fontSize="sm"
        overflow="hidden"
        textOverflow="ellipsis"
        textWrap="nowrap"
        maxWidth="130px"
      >
        {name.length ? name : 'New app'}
      </Text>
    </List.Item>
  )
}
