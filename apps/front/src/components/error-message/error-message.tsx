import { ReactNode } from 'react'
import { Text } from '@chakra-ui/react'

export function ErrorMessage({ children }: { children: ReactNode }) {
  return (
    <Text color="red.solid" fontSize="sm" role="alert">
      {children}
    </Text>
  )
}
