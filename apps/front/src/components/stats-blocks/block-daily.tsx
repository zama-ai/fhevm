import { lazy, Suspense } from 'react'
import {
  Card,
  FormatNumber,
  StatRoot,
  StatValueText,
  Text,
} from '@chakra-ui/react'
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

export function BlockDaily({
  data,
}: {
  data: Array<Record<string, string | number> & { value: number }>
}) {
  if (!data || data.length < 2)
    return (
      <Card.Root minH="83px" size="sm" width="300px">
        <Card.Body>
          <StatRoot pos="relative">
            <StatLabel>Daily usage</StatLabel>
            <Text fontSize="xs">- no usage yet -</Text>
          </StatRoot>
        </Card.Body>
      </Card.Root>
    )
  const today = data[data.length - 1].value ?? 0
  const yesterday = data[data.length - 2].value ?? 0
  const evolution = (today - yesterday) / today
  const PercentageComponent = evolution > 0 ? StatUpTrend : StatDownTrend

  return (
    <Card.Root minH="83px" size="sm" width="300px">
      <Card.Body>
        <StatRoot pos="relative">
          <StatLabel>Daily usage</StatLabel>
          <StatHelpText pos="absolute" top="0" right="0">
            <PercentageComponent>
              <FormatNumber
                value={evolution}
                style="percent"
                maximumFractionDigits={2}
                minimumFractionDigits={2}
              />
            </PercentageComponent>
          </StatHelpText>
          <StatValueText>
            {Math.ceil(Number(data[data.length - 1].value ?? 0) || 0)}
          </StatValueText>
          <Suspense fallback={null}>
            <Sparkline
              data={data}
              categories={['value', 'compareValue']}
              colors={['orange', 'gray']}
              height="60px"
              mx="-4"
            />
          </Suspense>
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
