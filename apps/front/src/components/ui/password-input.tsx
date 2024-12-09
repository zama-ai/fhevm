import type {
  ButtonProps,
  GroupProps,
  InputProps,
  StackProps,
} from '@chakra-ui/react'
import {
  Box,
  HStack,
  IconButton,
  Input,
  Stack,
  mergeRefs,
  useControllableState,
} from '@chakra-ui/react'
import { forwardRef, useRef, PointerEvent } from 'react'
import { Eye, EyeOff } from 'lucide-react'
import { InputGroup } from './input-group'

export interface PasswordVisibilityProps {
  defaultVisible?: boolean
  visible?: boolean
  onVisibleChange?: (visible: boolean) => void
  visibilityIcon?: { on: React.ReactNode; off: React.ReactNode }
}

export interface PasswordInputProps
  extends InputProps,
    PasswordVisibilityProps {
  rootProps?: GroupProps
}

export const PasswordInput = forwardRef<HTMLInputElement, PasswordInputProps>(
  function PasswordInput(props, ref) {
    const {
      rootProps,
      defaultVisible,
      visible: visibleProp,
      onVisibleChange,
      visibilityIcon = { on: <Eye />, off: <EyeOff /> },
      ...rest
    } = props

    const [visible, setVisible] = useControllableState({
      value: visibleProp,
      defaultValue: defaultVisible || false,
      onChange: onVisibleChange,
    })

    const inputRef = useRef<HTMLInputElement>(null)

    return (
      <InputGroup
        width="full"
        endElement={
          <VisibilityTrigger
            disabled={rest.disabled}
            onPointerDown={(ev: PointerEvent<HTMLButtonElement>) => {
              if (rest.disabled) return
              if (ev.button !== 0) return
              ev.preventDefault()
              setVisible(!visible)
            }}
          >
            {visible ? visibilityIcon.off : visibilityIcon.on}
          </VisibilityTrigger>
        }
        {...rootProps}
      >
        <Input
          {...rest}
          ref={mergeRefs(ref, inputRef)}
          type={visible ? 'text' : 'password'}
        />
      </InputGroup>
    )
  },
)

const VisibilityTrigger = forwardRef<HTMLButtonElement, ButtonProps>(
  function VisibilityTrigger(props, ref) {
    return (
      <IconButton
        tabIndex={-1}
        ref={ref}
        me="-2"
        aspectRatio="square"
        size="sm"
        variant="ghost"
        height="calc(100% - {spacing.2})"
        aria-label="Toggle password visibility"
        {...props}
      />
    )
  },
)

interface PasswordStrengthMeterProps extends StackProps {
  max?: number
  value: number
}

export const PasswordStrengthMeter = forwardRef<
  HTMLDivElement,
  PasswordStrengthMeterProps
>(function PasswordStrengthMeter(props, ref) {
  const { max = 4, value, ...rest } = props

  const percent = (value / max) * 100
  const { label, colorPalette } = getColorPalette(percent)

  return (
    <Stack align="flex-end" gap="1" ref={ref} {...rest}>
      <HStack width="full" ref={ref} {...rest}>
        {Array.from({ length: max }).map((_, index) => (
          <Box
            key={index}
            height="1"
            flex="1"
            rounded="sm"
            data-selected={index < value ? '' : undefined}
            layerStyle="fill.subtle"
            colorPalette="gray"
            _selected={{
              colorPalette,
              layerStyle: 'fill.solid',
            }}
          />
        ))}
      </HStack>
      {label && <HStack textStyle="xs">{label}</HStack>}
    </Stack>
  )
})

function getColorPalette(percent: number) {
  switch (true) {
    case percent < 26:
      return { label: 'Low', colorPalette: 'red' }
    case percent < 51:
      return { label: 'Medium', colorPalette: 'orange' }
    case percent < 76:
      return { label: 'High', colorPalette: 'green' }
    default:
      return { label: 'Great', colorPalette: 'blue' }
  }
}
