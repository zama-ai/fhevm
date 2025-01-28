import { Heading, Text } from '@chakra-ui/react'
import { Skeleton } from '@/components/ui/skeleton'

function randomGreeting() {
  const greetings = ['Welcome', 'Hello', 'Hi', 'Hey', 'Yo', 'Hiya', 'Howdy']
  return greetings[Math.floor(Math.random() * greetings.length)]
}

type OwnProps = { loading: boolean; name?: string }

export function HeroGreetings({ name, loading }: OwnProps) {
  return (
    <Skeleton asChild loading={loading} maxW="300px">
      <Heading mb="5">
        {randomGreeting()}
        <Text color="orange.400" as="span">
          {' '}
          {name ?? ''}!
        </Text>
      </Heading>
    </Skeleton>
  )
}
