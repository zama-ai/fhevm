import { useCallback } from 'react'
import { useTheme } from 'next-themes'

export function useColorMode() {
  const { resolvedTheme, setTheme } = useTheme()
  const toggleColorMode = useCallback(() => {
    setTheme(resolvedTheme === 'light' ? 'dark' : 'light')
  }, [resolvedTheme, setTheme])
  return {
    colorMode: resolvedTheme,
    setColorMode: setTheme,
    toggleColorMode,
  }
}

export function useColorModeValue<T>(light: T, dark: T) {
  const { colorMode } = useColorMode()
  return colorMode === 'light' ? light : dark
}
