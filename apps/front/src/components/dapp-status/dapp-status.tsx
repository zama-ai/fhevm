import { Badge, BadgeProps } from '@chakra-ui/react'

type OwnProps = {
  status: string
} & BadgeProps
export function DappStatus({ status, ...rest }: OwnProps) {
  let colorPalette: BadgeProps['colorPalette'] = 'purple'
  let variant: BadgeProps['variant'] = 'solid'
  switch (status) {
    case 'DRAFT':
      variant = 'surface'
      colorPalette = 'neutral'
      break
    case 'DEPLOYING':
      variant = 'subtle'
      colorPalette = 'orange'
      break
    case 'LIVE':
      colorPalette = 'green'
      break
    default:
      break
  }
  return (
    <Badge variant={variant} colorPalette={colorPalette} {...rest}>
      {status}
    </Badge>
  )
}
