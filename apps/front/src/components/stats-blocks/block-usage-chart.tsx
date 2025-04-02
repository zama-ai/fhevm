import { lazy, Suspense } from 'react'
import { Stack } from '@chakra-ui/react'

import { Box, Card, HStack, FormatNumber } from '@chakra-ui/react'

import {
  StatLabel,
  StatRoot,
  StatValueText,
  StatHelpText,
} from '@/components/ui/stat.js'

const PieChartComponent = lazy(() =>
  import('../pie/pie.js').then(module => ({
    default: module.PieChartComponent,
  })),
)

type OwnProps = {
  total: number
  operationStatsData: { name: string; value: number }[]
  operationStatsTotal: number
  encryptionStatsData: { name: string; value: number }[]
  encryptionStatsTotal: number
}
export const BlockUsageChart = ({
  total,
  operationStatsData,
  operationStatsTotal,
  encryptionStatsData,
  encryptionStatsTotal,
}: OwnProps) => {
  console.log({ operationStatsData, encryptionStatsData })
  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body>
        <StatRoot>
          <HStack>
            <Stack>
              <StatLabel>Total usage</StatLabel>

              <StatValueText>
                <FormatNumber
                  value={total}
                  notation="compact"
                  compactDisplay="short"
                />
              </StatValueText>
            </Stack>
            <Box width="240px">
              <Suspense fallback={null}>
                <Box position="relative" mt="2" w="100%">
                  <Stack direction="row" gap="2" zIndex="1">
                    <Box width="100%">
                      <PieChartComponent
                        w="100%"
                        data={operationStatsData}
                        height="80px"
                        containerHeight={60}
                        innerRadius={20}
                        outerRadius={30}
                        label={false}
                      />
                      <StatHelpText>
                        <FormatNumber
                          value={operationStatsTotal}
                          notation="compact"
                          compactDisplay="short"
                        />{' '}
                        FHE Operations
                      </StatHelpText>
                    </Box>
                    <Box width="100%">
                      <PieChartComponent
                        w="100%"
                        data={encryptionStatsData}
                        height="80px"
                        containerHeight={60}
                        innerRadius={20}
                        outerRadius={30}
                        label={false}
                      />
                      <StatHelpText>
                        <FormatNumber
                          value={encryptionStatsTotal}
                          notation="compact"
                          compactDisplay="short"
                        />{' '}
                        FHE Operations
                      </StatHelpText>
                    </Box>
                  </Stack>
                </Box>
              </Suspense>
            </Box>
          </HStack>
        </StatRoot>
      </Card.Body>
    </Card.Root>
  )
}
