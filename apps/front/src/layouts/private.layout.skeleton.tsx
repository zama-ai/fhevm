import { Box, Flex, Stack } from '@chakra-ui/react'
import { Skeleton, SkeletonText } from '@/components/ui/skeleton'

export function PrivateLayoutSkeleton() {
  return (
    <Box className="layout-private">
      <Skeleton height="70px" rounded={0} />
      <Flex direction="row" wrap="nowrap" justify="flex-start" align="stretch">
        <Box display={{ base: 'none', lg: 'block' }}>
          <Box
            className="navigation"
            flexBasis="300px"
            flexGrow="0"
            flexShrink="0"
            borderRightStyle="solid"
            borderRightWidth="1px"
            borderRightColor="gray.200"
            px="50px"
            py="30px"
          >
            <Stack gap="5">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} width="100px" height="20px" />
              ))}
            </Stack>
          </Box>
        </Box>
        <Box p="40px" flexGrow="1">
          <Skeleton width="130px" height="20px" mb="20px" />
          <SkeletonText />
        </Box>
      </Flex>
    </Box>
  )
}
