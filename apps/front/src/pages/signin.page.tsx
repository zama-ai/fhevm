import { gql, useMutation } from '@apollo/client'

import { SignInMutation } from '@/__generated__/graphql'
import { SigninForm } from '@/components/signin-form/signin-form'

const SIGN_IN = gql`
  mutation SignIn($email: String!, $password: String!) {
    login(input: { email: $email, password: $password }) {
      token
    }
  }
`

export function SigninPage() {
  const [signInMutation, { data, loading, error }] =
    useMutation<SignInMutation>(SIGN_IN)

  if (data) {
    localStorage.setItem('token', data.login.token)
  }
  return (
    <>
      {loading ? <p>Loading...</p> : null}
      {error ? <p>Error :(</p> : null}
      {data ? <pre>{JSON.stringify(data, null, 2)}</pre> : null}
      <button
        onClick={() => {
          signInMutation({
            variables: { email: 'gaspard@gmail.com', password: 'a' },
          })
        }}
      >
        gql
      </button>
      <SigninForm />
    </>
  )
}
