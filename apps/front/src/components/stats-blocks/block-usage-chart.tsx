import { useState, useRef, useEffect } from 'react'

import { Box, Flex, Card, HStack, Grid, Text } from '@chakra-ui/react'
import {
  StatLabel,
  StatRoot,
  StatValueText,
  StatHelpText,
} from '@/components/ui/stat'

function Ball({ color = 'neutral.200' }) {
  return <Box w=".4em" h=".4em" bg={color} rounded="full" flexShrink={0} />
}

const randomValues = [
  151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140,
  36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
  75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237,
  149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48,
  27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105,
  92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73,
  209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
  164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
  147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 5,
]
const scale = window.devicePixelRatio

function draw(ctx: CanvasRenderingContext2D, scale: number) {
  ctx.lineWidth = 3 * scale
  const colors = ['#ffd208', '#4f7eec', '#f228f2']
  for (let c = 0; c < colors.length; c++) {
    ctx.strokeStyle = colors[c]
    for (let i = 0; i < randomValues.length; i++) {
      ctx.beginPath()
      ctx.moveTo(10 + i * 5 * scale, c * 50 + 25 + randomValues[i] / 4)
      ctx.lineCap = 'round'
      ctx.lineTo(10 + i * 5 * scale, 80 * scale)
      ctx.stroke()
    }
  }
}

function Chart() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null)

  const width = 300
  const height = 80

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const context = canvas.getContext('2d')
    if (!context) return
    draw(context, scale)
  }, [])
  return (
    <canvas
      ref={canvasRef}
      width={Math.floor(width * scale)}
      height={Math.floor(height * scale)}
      style={{ width, height }}
    ></canvas>
  )
}

export const BlockUsageChart = () => {
  const textColor = 'gray.700'
  const [daily] = useState<number>(-1)

  return (
    <Card.Root minH="83px" size="sm">
      <Card.Body>
        <Box>
          <Flex align="center">
            <StatRoot mr={2} h="60px" w="80px" flexGrow={0}>
              <Box w="80px"></Box>
              {daily === -1 && (
                <StatLabel overflow="hidden" textWrap="ellipsis">
                  Daily Usage
                </StatLabel>
              )}
              <StatValueText color={textColor}>
                {daily !== -1 ? daily : 223}
              </StatValueText>

              {daily !== -1 && (
                <StatHelpText>
                  {new Date(Date.now() - 1000 * 86400 * daily)
                    .toISOString()
                    .substring(0, 10)}
                </StatHelpText>
              )}
            </StatRoot>
            <Box h="60px" flexGrow={1} position="relative">
              <HStack gap="2px" h="60px" alignItems="flex-end">
                <Box w="300px" h="60px">
                  <Chart />
                </Box>

                <Grid
                  alignItems="center"
                  templateColumns={'.6em 1fr'}
                  templateRows="repeat(3, 1fr)"
                  columnGap="1"
                  rowGap="0px"
                  ml="2"
                >
                  <Ball color="#ffd208" />
                  <Text fontSize="2xs">FHE computations</Text>
                  <Ball color="#4f7eec" />
                  <Text fontSize="2xs">Decryptions</Text>
                  <Ball color="#f228f2" />
                  <Text fontSize="2xs">Re-encryptions</Text>
                </Grid>
              </HStack>
            </Box>
          </Flex>
        </Box>
      </Card.Body>
    </Card.Root>
  )
}
