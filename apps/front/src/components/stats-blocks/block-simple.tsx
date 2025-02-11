import { Card } from '@chakra-ui/react'
import { StatLabel, StatRoot, StatValueText } from '@/components/ui/stat.js'

type OwnProps = {
  title: string
  amount: number
}

export const BlockSimple = ({ title, amount }: OwnProps) => {
  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body>
        <StatRoot>
          <StatLabel>{title}</StatLabel>
          <StatValueText value={amount} />
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
