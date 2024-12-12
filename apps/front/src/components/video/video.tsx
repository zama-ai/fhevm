import { Box, BoxProps } from '@chakra-ui/react'
type OwnProps = {
  src: string
} & BoxProps

export function Video({ src, ...props }: OwnProps) {
  return (
    <Box
      {...props}
      style={{
        position: 'relative',
        width: '100%',
        height: '0',
        paddingBottom: '56.27198%',
      }}
    >
      <iframe
        style={{
          position: 'absolute',
          top: '0',
          left: '0',
          width: '100%',
          height: '100%',
        }}
        width="500"
        height="294"
        src={src}
        allow="encrypted-media; web-share"
      ></iframe>
      {src}
    </Box>
  )
}
