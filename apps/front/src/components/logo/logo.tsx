import { RefAttributes } from 'react'
import zamaLogo from '@/assets/zama-logo.svg'

type OwnProps = Omit<
  RefAttributes<HTMLImageElement>,
  'src' | 'alt' | 'width' | 'class'
> & {
  width?: number
  height?: number
}

export function Logo({ width, ...props }: OwnProps) {
  return (
    <img
      src={zamaLogo}
      width={width ?? 100}
      className="logo"
      alt="Zama logo"
      {...props}
    />
  )
}
