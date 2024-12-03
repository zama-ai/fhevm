import { useContext } from 'react'
import { Box, Stack, Text } from '@chakra-ui/react'
import { TitleContext } from '@/components/title-context/title-context'

export function Navigation() {
  const { title } = useContext(TitleContext)
  return (
    <Box className="navigation">
      <Stack>
        <Box className="editable"></Box>
        <Box>
          <Text>Navigation</Text>
          <Text>{title}</Text>
        </Box>
      </Stack>
    </Box>
  )
}
