import { Box, Button } from '@chakra-ui/react'

type OwnProps = {
  createDapp: () => void
}
export function DappsList({ createDapp }: OwnProps) {
  return (
    <Box py="5">
      <Button onClick={createDapp}>+ Create a new dAapp</Button>
    </Box>
  )
}
