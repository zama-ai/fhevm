import { forwardRef } from 'react'
import { IconButton as ChakraIconButton } from '@chakra-ui/react'
import type { ButtonProps } from '@chakra-ui/react'
import { CloseIcon } from '../icons/icons'

export type CloseButtonProps = ButtonProps

export const CloseButton = forwardRef<HTMLButtonElement, CloseButtonProps>(
  function CloseButton(props, ref) {
    return (
      <ChakraIconButton variant="ghost" aria-label="Close" ref={ref} {...props}>
        {props.children ?? <CloseIcon />}
      </ChakraIconButton>
    )
  },
)
