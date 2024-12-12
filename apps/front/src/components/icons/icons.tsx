import { RefAttributes } from 'react'
import { Icon, IconProps } from '@chakra-ui/react'
import {
  BookOpenText,
  ChevronDown,
  Clock,
  LayoutGrid,
  LogOut,
  Menu,
  Monitor,
  Plus,
  Settings,
  Smile,
  X,
  LucideProps,
} from 'lucide-react'

type OwnProps = IconProps & RefAttributes<SVGSVGElement> & LucideProps

export const ChevronDownIcon = (props: OwnProps) => (
  <Icon {...props}>
    <ChevronDown />
  </Icon>
)

export const CloseIcon = (props: OwnProps) => (
  <Icon {...props}>
    <X />
  </Icon>
)

export const CommunityIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Smile />
  </Icon>
)

export const DocumentationIcon = (props: OwnProps) => (
  <Icon {...props}>
    <BookOpenText />
  </Icon>
)

export const DashboardIcon = (props: OwnProps) => (
  <Icon {...props}>
    <LayoutGrid />
  </Icon>
)

export const HamburgerIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Menu />
  </Icon>
)

export const LearnIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Monitor />
  </Icon>
)

export const LogOutIcon = (props: OwnProps) => (
  <Icon {...props}>
    <LogOut />
  </Icon>
)

export const NewIcon = ({ strokeWidth, ...props }: OwnProps) => (
  <Icon {...props}>
    <Plus strokeWidth={strokeWidth} />
  </Icon>
)

export const SettingsIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Settings />
  </Icon>
)

export const WaitIcon = (props: OwnProps) => (
  <Icon {...props}>
    <Clock />
  </Icon>
)
