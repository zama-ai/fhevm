import { Box, Text } from '@chakra-ui/react'
import { Alert } from '../ui/alert'

export function TutorialName() {
  return (
    <>
      <Box
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
          src="https://www.youtube.com/embed/1FtbyHZwNX4?&theme=dark&keyboard=1&autohide=1"
          allow="encrypted-media; web-share"
        ></iframe>
      </Box>

      <Text fontSize="xs" my="1">
        <b>Tutorial:</b> Use this solidity code in Remix and deploy it on
        Sepolia.
      </Text>
      <Alert status="warning" title="Some warning to be redacted" my="5">
        This code is signed, don't share it with others. Ask Roger to rephrase
        this warning.
      </Alert>
    </>
  )
}
