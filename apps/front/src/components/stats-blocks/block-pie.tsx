import { lazy, Suspense } from 'react'
import { Box, Card } from '@chakra-ui/react'
import { StatLabel, StatRoot, StatValueText } from '@/components/ui/stat'

const PieChartComponent = lazy(() =>
  import('../pie/pie.js').then(module => ({
    default: module.PieChartComponent,
  })),
)

type OwnProps = {
  title: string
  total: number
  data: { name: string; value: number }[]
}

export function BlockPie({ title, total, data }: OwnProps) {
  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body p="4">
        <StatRoot pos="relative">
          <StatLabel>{title}</StatLabel>
          <StatValueText
            width="100%"
            textAlign="center"
            display="block"
            mt="5px"
          >
            {total}
          </StatValueText>
          <Suspense fallback={null}>
            <Box position="relative" mt="2" w="100%">
              <PieChartComponent
                position="absolute"
                top="-56px"
                left="0"
                w="100%"
                data={data}
                height="60px"
                containerHeight={60}
                innerRadius={25}
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
