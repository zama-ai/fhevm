import { lazy, Suspense } from 'react'
import { Box, Card } from '@chakra-ui/react'
import { StatLabel, StatRoot, StatValueText } from '@/components/ui/stat'

const PieChartComponent = lazy(() =>
  import('../pie/pie.js').then(module => ({
    default: module.PieChartComponent,
  })),
)

type OwnProps = { total: number; data: { name: string; value: number }[] }

export function BlockPie({ total, data }: OwnProps) {
  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body p="4">
        <StatRoot pos="relative">
          <StatLabel>FHE Events</StatLabel>
          <StatValueText width="100%" textAlign="center" display="block">
            {total}
          </StatValueText>
          <Suspense fallback={null}>
            <Box position="relative" mt="2" w="100%" border="1px solid pink">
              <PieChartComponent
                position="absolute"
                top="-60px"
                left="0"
                w="100%"
                border="1px solid red"
                data={data}
                height="60px"
                containerHeight={60}
                innerRadius={20}
                outerRadius={30}
                label={false}
              />
            </Box>
          </Suspense>
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
