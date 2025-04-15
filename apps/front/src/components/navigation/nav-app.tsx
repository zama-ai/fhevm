import { List } from '@chakra-ui/react'
import { NavLink } from '@/components/ui/link.js'
import { NavAppBlock } from './nav-app-block.js'

type OwnProps = {
  id: string
  name: string
  status: string
}

export function NavApp({ id, name, status }: OwnProps) {
  const color = ['LIVE', 'DEPLOYING'].includes(status)
    ? 'green.200'
    : 'gray.300'
  return (
    <NavLink to={`/dapp/${id}`}>
      {({ isActive }) => (
        <PureNavApp name={name} color={color} isActive={isActive} />
      )}
    </NavLink>
  )
}

type PureNavAppProps = {
  name: string
  color: string
  isActive: boolean
}

export function PureNavApp({ name, color, isActive }: PureNavAppProps) {
  return (
    <List.Item
      className="group"
      bg={isActive ? 'brand.subtle' : 'inherit'}
      width="100%"
      _dark={{ bg: isActive ? 'brand.subtle' : 'inherit' }}
      rounded="md"
      pl="2"
    >
      <NavAppBlock name={name} color={color} isActive={isActive} />
    </List.Item>
  )
}
