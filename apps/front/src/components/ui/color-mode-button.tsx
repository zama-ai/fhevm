import { forwardRef } from 'react'
import { ClientOnly, IconButton, Skeleton } from '@chakra-ui/react'
import type { IconButtonProps } from '@chakra-ui/react'
import { Moon, Sun } from 'lucide-react'

import { useColorMode } from '@/hooks/use-color-mode.js'

export function ColorModeIcon() {
  const { colorMode } = useColorMode()
  return colorMode === 'dark' ? <Sun /> : <Moon />
}

type ColorModeButtonProps = Omit<IconButtonProps, 'aria-label'> & {}

export const ColorModeButton = forwardRef<
  HTMLButtonElement,
  ColorModeButtonProps
>(function ColorModeButton(props, ref) {
  const { toggleColorMode } = useColorMode()
  return (
    <ClientOnly fallback={<Skeleton boxSize="8" />}>
      <IconButton
        onClick={toggleColorMode}
        variant="ghost"
        aria-label="Toggle color mode"
        size="sm"
        ref={ref}
        {...props}
        css={{
          _icon: {
            width: '4',
            height: '4',
          },
        }}
      >
        <ColorModeIcon />
      </IconButton>
    </ClientOnly>
  )
})
