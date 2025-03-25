import { lazy, Suspense } from 'react'
import { Card } from '@chakra-ui/react'
import { StatLabel, StatRoot, StatValueText } from '@/components/ui/stat'

const PieChartComponent = lazy(() =>
  import('../pie/pie.js').then(module => ({
    default: module.PieChartComponent,
  })),
)

export function BlockPie() {
  const total = 100
  const data = [
    { name: 'A', value: 40 },
    { name: 'B', value: 300 },
    { name: 'C', value: 300 },
  ]
  return (
    <Card.Root width="300px">
      <Card.Body p="4">
        <StatRoot pos="relative">
          <StatLabel>FHE Operations</StatLabel>
          <StatValueText>{total}</StatValueText>
          <Suspense fallback={null}>
            <PieChartComponent
              data={data}
              height="60px"
              mx="-4"
              innerRadius={20}
              outerRadius={30}
              label={false}
            />
          </Suspense>
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
