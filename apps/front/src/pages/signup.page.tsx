import { SignupForm } from '@/components/signup-form/signup-form'

export function SignupPage() {
  return (
    <SignupForm
      onSubmit={() => {}}
      loading={false}
      invitationKey="a"
      email="miaouss@poke.mon"
    />
  )
}
