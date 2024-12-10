import { Heading, Text } from '@chakra-ui/react'

function randomGreeting() {
  const greetings = [
    'Welcome',
    'Hello',
    'Hi',
    'Hey',
    'Yo',
    'Sup',
    'Hiya',
    'Howdy',
  ]
  return greetings[Math.floor(Math.random() * greetings.length)]
}

type OwnProps = {
  name: string
}
export function HeroGreetings({ name }: OwnProps) {
  return (
    <Heading>
      {randomGreeting()}
      <Text color="orange.400" as="span">
        {' '}
        {name}!
      </Text>
    </Heading>
  )
}
