import { useParams } from 'react-router'
import { RequestResetPassword } from './request-reset-password'
import { ResetPassword } from './reset-password'

// The page can be in two possible state:
// - in case there is a token, we show the reset password form
// - in case there is no token, we show the request reset password form
export function ResetPasswordPage() {
  const params = useParams()
  return params.token ? (
    <ResetPassword token={params.token} />
  ) : (
    <RequestResetPassword />
  )
}
