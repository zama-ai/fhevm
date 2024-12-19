import { List, Text } from '@chakra-ui/react'
import { Circle } from 'lucide-react'
import { NavLink } from '@/components/ui/link'

type NavAppBlockProps = {
  name: string
  color: string
  isActive?: boolean
}

function NavAppBlock({ name, color, isActive }: NavAppBlockProps) {
  return (
    <>
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
        fontWeight={isActive ? 'bold' : 'normal'}
      >
        {name.length ? name : 'New app'}
      </Text>
    </>
  )
}

type NavAppProps = {
  id: string
  name: string
  status: string
}

export function NavApp({ id, name, status }: NavAppProps) {
  const color = ['LIVE', 'DEPLOYING'].includes(status)
    ? 'green.200'
    : 'gray.300'
  const link = ['LIVE', 'DEPLOYING'].includes(status)
    ? `/dapp/${id}`
    : `/create/2/${id}`
  return (
    <List.Item>
      <NavLink to={link} className="group">
        {({ isActive }) => (
          <NavAppBlock name={name} color={color} isActive={isActive} />
        )}
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
      <NavAppBlock name={name} color="gray.300" />
    </List.Item>
  )
}
