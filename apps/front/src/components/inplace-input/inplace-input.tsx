import { useState } from 'react'
import { z } from 'zod'
import {
  Editable,
  Popover,
  IconButton,
  Heading,
  useDisclosure,
} from '@chakra-ui/react'
import { Check, MessageSquareWarning, PencilLine, X } from 'lucide-react'

type OwnProps = {
  title: string
  onChange?: ({ value }: { value: string }) => void
  onUpdate?: ({ value }: { value: string }) => void
  error: string | null
  placeholder?: string
}

const valueSchema = z.string().min(1, 'Cannot be empty')

export const InplaceInput = ({
  title,
  onUpdate,
  onChange,
  error,
  placeholder,
}: OwnProps) => {
  const { open, onOpen, onClose } = useDisclosure()
  const [validationError, setValidationError] = useState<string | null>(null)

  const handleUpdate = ({ value }: { value: string }) => {
    try {
      valueSchema.parse(value.trim())
    } catch (error) {
      if (error instanceof z.ZodError) {
        setValidationError(error.errors[0].message)
        return
      }
      setValidationError('Dapp name cannot be empty')
      return
    }
    if (onUpdate) {
      onUpdate({ value })
    }
    setValidationError(null)
  }

  return (
    <form>
      <Editable.Root
        defaultValue={title}
        finalFocusEl={() => null}
        maxLength={64}
        onValueChange={onChange}
        onValueCommit={handleUpdate}
        placeholder={placeholder}
        required
      >
        {(error || validationError) && (
          <Popover.Root>
            <Popover.Trigger asChild>
              <IconButton variant="ghost" size="xs" color={'red.500'}>
                <MessageSquareWarning />
              </IconButton>
            </Popover.Trigger>
            <Popover.Positioner>
              <Popover.Content>
                <Popover.CloseTrigger />
                <Popover.Arrow>
                  <Popover.ArrowTip />
                </Popover.Arrow>
                <Popover.Body>
                  <Popover.Title>
                    The modification may not have been saved:
                  </Popover.Title>
                  {error}
                  {validationError}
                </Popover.Body>
              </Popover.Content>
            </Popover.Positioner>
          </Popover.Root>
        )}
        <Heading size="xl">
          <Editable.Preview onMouseEnter={onOpen} onMouseLeave={onClose} />
          <Editable.Input border={'none'} />
        </Heading>
        <Editable.Control>
          <Editable.EditTrigger asChild>
            <IconButton
              variant="ghost"
              size="xs"
              display={open ? 'inline-block' : 'none'}
            >
              <PencilLine />
            </IconButton>
          </Editable.EditTrigger>
          <Editable.CancelTrigger asChild>
            <IconButton variant="outline" size="xs">
              <X />
            </IconButton>
          </Editable.CancelTrigger>
          <Editable.SubmitTrigger asChild>
            <IconButton variant="outline" size="xs">
              <Check />
            </IconButton>
          </Editable.SubmitTrigger>
        </Editable.Control>
      </Editable.Root>
    </form>
  )
}
