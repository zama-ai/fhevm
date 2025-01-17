import { lazy } from 'react'
import { Card, StatRoot, StatValueText } from '@chakra-ui/react'
import {
  StatLabel,
  StatHelpText,
  StatUpTrend,
  StatDownTrend,
} from '@/components/ui/stat'

const Sparkline = lazy(() =>
  import('../sparkline/sparkline.js').then(module => ({
    default: module.Sparkline,
  })),
)

export function BlockSparkline() {
  const value = Number(data[data.length - 1].value ?? 0)
  const compareValue = Number(data[data.length - 1].compareValue ?? 0)
  const percentage = Math.round(((value - compareValue) / value) * 100)

  return (
    <Card.Root width="300px">
      <Card.Body p="4">
        <StatRoot pos="relative">
          <StatLabel>Computations</StatLabel>
          <StatHelpText pos="absolute" top="0" right="0">
            {percentage > 0 ? (
              <StatUpTrend>{percentage}%</StatUpTrend>
            ) : (
              <StatDownTrend>{percentage}%</StatDownTrend>
            )}
          </StatHelpText>
          <StatValueText>
            {Math.ceil(data[data.length - 1].value ?? 0)}
          </StatValueText>
          <Sparkline
            data={data}
            categories={['value', 'compareValue']}
            colors={['orange', 'gray']}
            height="60px"
            mx="-4"
          />
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}

const pseudorandom: Array<Array<number>> = [
  [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140],
  [36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120],
]
const data: Array<{ value: number; compareValue: number }> =
  pseudorandom[0].map((value: number, index: number) => ({
    value,
    compareValue: pseudorandom[1][index],
  }))
