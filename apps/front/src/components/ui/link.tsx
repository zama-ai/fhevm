import { ReactNode, RefAttributes } from 'react'
import { NavLink, NavLinkProps } from 'react-router'
import { Link as ChakraLink, LinkProps } from '@chakra-ui/react'

type OwnProps = LinkProps &
  NavLinkProps &
  RefAttributes<HTMLAnchorElement> & { children: ReactNode }

export function Link({ unstyled, to, children, variant, ...props }: OwnProps) {
  return (
    <ChakraLink unstyled={unstyled} variant={variant} asChild>
      <NavLink to={to} {...props}>
        {children}
      </NavLink>
    </ChakraLink>
  )
}
