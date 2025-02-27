import { Box } from '@chakra-ui/react'

export function Dot() {
  return (
    <Box
      w="4"
      h="4"
      borderRadius="full"
      bg="gray.300"
      style={{
        background: 'conic-gradient(from 0deg, #b2daf9 180deg, #2096f3 0)',
        borderRadius: '50%',
        animation: 'spin 1s linear infinite',
      }}
    />
  )
}

export function GreenDot() {
  return <Box w="4" h="4" borderRadius="full" bg="green.300" />
}

export function AnimatedDot() {
  return <Box w="4" h="4" borderRadius="full" bg="gray.300" />
}
