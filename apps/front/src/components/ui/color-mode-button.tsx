import type { IconButtonProps } from '@chakra-ui/react'
import { ClientOnly, IconButton, Skeleton } from '@chakra-ui/react'
import { forwardRef } from 'react'
import { Moon, Sun } from 'lucide-react'
import { useColorMode } from '@/hooks/use-color-mode'

export function ColorModeIcon() {
  const { colorMode } = useColorMode()
  return colorMode === 'light' ? <Sun /> : <Moon />
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
            width: '5',
            height: '5',
          },
        }}
      >
        <ColorModeIcon />
      </IconButton>
    </ClientOnly>
  )
})
