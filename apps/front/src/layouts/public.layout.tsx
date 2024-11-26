import { Outlet } from 'react-router'
import { Logo } from '@/components/logo/logo'

export function PublicLayout() {
  return (
    <div>
      <div>
        <Logo />
      </div>
      <Outlet />
    </div>
  )
}
