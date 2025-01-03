import { RefAttributes } from 'react'
import {
  Link as RouterLink,
  LinkProps as RouterLinkProps,
  NavLink as RouterNavLink,
  NavLinkProps as RouterNavLinkProps,
} from 'react-router'
import {
  Link as ChakraLink,
  LinkProps as ChakraLinkProps,
  LinkOverlay as ChakraLinkOverlay,
  LinkOverlayProps as ChakraLinkOverlayProps,
} from '@chakra-ui/react'

type LinkProps = ChakraLinkProps &
  RouterLinkProps &
  RefAttributes<HTMLAnchorElement>
/**
 * Chakra + Router Link
 * @usage <Link to="/home">Home</Link>
 */
export function Link({ unstyled, to, children, variant, ...props }: LinkProps) {
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

type NavLinkProps = Omit<ChakraLinkProps, 'children'> &
  RouterNavLinkProps &
  RefAttributes<HTMLAnchorElement>
export function NavLink({
  unstyled,
  to,
  children,
  variant,
  ...props
}: NavLinkProps) {
  return (
    <ChakraLink unstyled={unstyled} variant={variant} asChild>
      <RouterNavLink to={to} {...props}>
        {children}
      </RouterNavLink>
    </ChakraLink>
  )
}

type LinkOverlayProps = ChakraLinkOverlayProps &
  RouterLinkProps &
  RefAttributes<HTMLAnchorElement>
export function LinkOverlay({ to, children, ...props }: LinkOverlayProps) {
  return (
    <ChakraLinkOverlay asChild>
      <RouterLink to={to} {...props}>
        {children}
      </RouterLink>
    </ChakraLinkOverlay>
  )
}
