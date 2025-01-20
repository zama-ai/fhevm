import { Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'
import { useNavigate } from 'react-router'
import { useEffect } from 'react'

export function DefaultPage() {
  const navigate = useNavigate()
  const logged = !!localStorage.getItem('token')

  useEffect(() => {
    if (logged) {
      navigate('/dashboard')
    } else {
      navigate('/signin')
    }
  }, [logged, navigate])
  return (
    <Stack gap="4">
      <Text>
        <Link to="/signin">signin</Link>
      </Text>
    </Stack>
  )
}
