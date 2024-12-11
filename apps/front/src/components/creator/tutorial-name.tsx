import { Text } from '@chakra-ui/react'
import { Alert } from '../ui/alert'

export function TutorialName() {
  return (
    <>
      <iframe
        width="100%"
        height="200"
        src="https://www.youtube.com/embed/1FtbyHZwNX4?si=2aBiv5N58ffKXD_L"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      ></iframe>
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
