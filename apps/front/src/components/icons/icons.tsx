import { RefAttributes } from "react"
import { Icon, IconProps } from "@chakra-ui/react"

import { Menu, X, ChevronDown, ChevronRight } from "lucide-react"

type OwnProps = IconProps & RefAttributes<SVGSVGElement>

export const HamburgerIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Menu />
  </Icon>
)

export const CloseIcon = (props: OwnProps) => (
  <Icon {...props}>
    <X />
  </Icon>
)

export const ChevronDownIcon = () => (
  <Icon fontSize="40px" color="tomato">
    <ChevronDown />
  </Icon>
)

export const ChevronRightIcon = () => (
  <Icon fontSize="40px" color="tomato">
    <ChevronRight />
  </Icon>
)
