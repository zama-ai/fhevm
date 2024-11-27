import { Logo } from '@/components/logo/logo'

type PublicLayoutProps = {
  children: React.ReactNode
}
export function PublicLayout({ children }: PublicLayoutProps) {
  return (
    <div>
      <div>
        <Logo />
      </div>
      {children}
    </div>
  )
}
