import { RefAttributes } from 'react'
import {
  Link as RouterLink,
  LinkProps as RouterLinkProps,
  NavLink as RouterNavLink,
  NavLinkProps as RouterNavLinkProps,
} from 'react-router'
import { Link as ChakraLink, LinkProps } from '@chakra-ui/react'

type OwnProps = LinkProps & RouterLinkProps & RefAttributes<HTMLAnchorElement>
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
 * @usage &lt;NavLink to="/home">{({ isActive}) => isActive ? <>→ Home</> : <>Home</>}</NavLink>
  }
 */

type SuperNavLinkProps = Omit<LinkProps, 'children'> &
  RouterNavLinkProps &
  RefAttributes<HTMLAnchorElement>
export function NavLink({
  unstyled,
  to,
  children,
  variant,
  ...props
}: SuperNavLinkProps) {
  return (
    <ChakraLink unstyled={unstyled} variant={variant} asChild>
      <RouterNavLink to={to} {...props}>
        {children}
      </RouterNavLink>
    </ChakraLink>
  )
}
