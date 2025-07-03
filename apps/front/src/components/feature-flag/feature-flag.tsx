import { ReactNode } from 'react'
import { useFeatureFlag, Flag } from '@/hooks/use-feature-flag'

export function FeatureFlag({
  is,
  children,
  not = false,
}: {
  is: Flag
  children: ReactNode
  not?: boolean
}) {
  const flag = useFeatureFlag(is)
  const active = not ? !flag : flag

  if (active) {
    return <>{children}</>
  }
  return null
}
