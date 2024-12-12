import { Box, Text } from '@chakra-ui/react'
import { Video } from '@/components/video/video'
export function TutorialAddress() {
  return (
    <Box>
      <Video
        w="1/2"
        src="https://www.youtube.com/embed/cN7XXLyb1Fo?si=ST1nI5MYxMp6kYoE"
      />

      <Text fontSize="xs" my="1">
        <b>Tutorial:</b> Deploy your dApp on the blockchain
      </Text>
    </Box>
  )
}
