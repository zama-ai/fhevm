import { Box, Text } from '@chakra-ui/react'

export function TutorialAddress() {
  return (
    <Box>
      <Box
        w="1/2"
        style={{
          position: 'relative',
          width: '100%',
          height: '0',
          paddingBottom: '56.27198%',
        }}
      >
        <iframe
          style={{
            position: 'absolute',
            top: '0',
            left: '0',
            width: '100%',
            height: '100%',
          }}
          width="500"
          height="294"
          src="https://www.youtube.com/embed/cN7XXLyb1Fo?si=ST1nI5MYxMp6kYoE"
          allow="encrypted-media; web-share"
        ></iframe>
      </Box>

      <Text fontSize="xs" my="1">
        <b>Tutorial:</b> Deploy your dApp on the blockchain
      </Text>
    </Box>
  )
}
