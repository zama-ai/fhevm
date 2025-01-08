import { Card, HStack, StatHelpText } from '@chakra-ui/react'
import {
  StatDownTrend,
  StatLabel,
  StatRoot,
  StatUpTrend,
  StatValueText,
} from '@/components/ui/stat.js'

type OwnProps = {
  title: string
  amount: number
  percentage: number
  description: string
}

export const BlockUaw = ({
  title,
  amount,
  percentage,
  description,
}: OwnProps) => {
  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body>
        <StatRoot>
          <StatLabel>{title}</StatLabel>
          <HStack>
            <StatValueText value={amount} />
            {percentage < 0 ? (
              <StatDownTrend>{percentage}%</StatDownTrend>
            ) : (
              <StatUpTrend>{percentage}%</StatUpTrend>
            )}
          </HStack>
          <StatHelpText mb="2">{description}</StatHelpText>
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
