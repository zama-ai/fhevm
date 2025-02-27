import { Box } from '@chakra-ui/react'

type OwnProps = {
  variant?: 'default' | 'green' | 'animated'
}
export function InputGroupDot({ variant }: OwnProps) {
  const bg =
    variant === 'green'
      ? 'green.300'
      : variant === 'animated'
        ? '#b2daf9'
        : 'gray.300'

  return (
    <Box
      w="4"
      h="4"
      borderRadius="full"
      bg={bg}
      style={
        variant === 'animated'
          ? {
              background:
                'conic-gradient(from 0deg, #b2daf9 180deg, #2096f3 0)',
              borderRadius: '50%',
              animation: 'spin 1s linear infinite',
            }
          : {}
      }
    />
  )
}
