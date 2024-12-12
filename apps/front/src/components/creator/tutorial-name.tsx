import { Text } from '@chakra-ui/react'
import { Alert } from '@/components/ui/alert'
import { Video } from '@/components/video/video'

export function TutorialName() {
  return (
    <>
      <Video
        w="1/2"
        src="https://www.youtube.com/embed/1FtbyHZwNX4?&theme=dark&keyboard=1&autohide=1"
      />

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
