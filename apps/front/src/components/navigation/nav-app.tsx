import { List } from '@chakra-ui/react'
import { NavLink } from '@/components/ui/link.js'
import { NavAppBlock } from './nav-app-block.js'

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

type NewNavAppProps = {
  name: string
}

export function NewNavApp({ name }: NewNavAppProps) {
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
