import { useEffect } from 'react'
import { useLoaderData } from 'react-router'
import { useNavigate } from 'react-router'
import { Text } from '@chakra-ui/react'
import { ValidationTokenMutation } from '@/__generated__/graphql'

// import { InvitationTokenQuery } from '@/__generated__/graphql.js'

export function ValidationPage() {
  const {
    confirmEmail: { token },
  } = useLoaderData<ValidationTokenMutation>()

  const navigate = useNavigate()

  useEffect(() => {
    if (token) {
      localStorage.setItem('token', token)
      navigate('/dashboard/')
    }
  }, [token, navigate])

  return <Text>You are being redirected…</Text>
}
