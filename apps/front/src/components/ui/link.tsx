import { ReactNode, RefAttributes } from 'react'
import {
  Link as RouterLink,
  NavLink as RouterNavLink,
  NavLinkProps,
} from 'react-router'
import { Link as ChakraLink, LinkProps } from '@chakra-ui/react'

type OwnProps = LinkProps &
  NavLinkProps &
  RefAttributes<HTMLAnchorElement> & { children: ReactNode }
/**
 * Chakra + Router Link
 * @usage <Link to="/home">Home</Link>
 */
export function Link({ unstyled, to, children, variant, ...props }: OwnProps) {
  return (
    <ChakraLink unstyled={unstyled} variant={variant} asChild>
      <RouterLink to={to} {...props}>
        {children}
      </RouterLink>
    </ChakraLink>
  )
}

/**
 * Navigation links that need to render an active state.
 * @usage &lt;NavLink to="/home">(({ isActive}) => isActive ? <>→ Home</> : <>Home</> )}</NavLink>
  }
 */
export function NavLink({
  unstyled,
  to,
  children,
  variant,
  ...props
}: OwnProps) {
  return (
    <ChakraLink unstyled={unstyled} variant={variant} asChild>
      <RouterNavLink to={to} {...props}>
        {children}
      </RouterNavLink>
    </ChakraLink>
  )
}
