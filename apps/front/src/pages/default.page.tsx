import { Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'
import { useNavigate } from 'react-router'
import { useEffect } from 'react'

export function DefaultPage() {
  const navigate = useNavigate()

  useEffect(() => {
    const logged = !!localStorage.getItem('token')
    if (logged) {
      navigate('/dashboard')
    } else {
      navigate('/signin')
    }
  }, [navigate])
  return (
    <Stack gap="4">
      <Text>
        <Link to="/signin">signin</Link>
      </Text>
    </Stack>
  )
}
