import { Editable, IconButton, Heading, useDisclosure } from "@chakra-ui/react"
import { Check, PencilLine, X } from "lucide-react"

type OwnProps = {
  title: string
  onChange: ({ value }: { value: string }) => void
  onUpdate: ({ value }: { value: string }) => void
}

export const InplaceInput = ({ title, onChange, onUpdate }: OwnProps) => {
  const { open, onOpen, onClose } = useDisclosure()
  return (
    <Editable.Root
      defaultValue={title}
      finalFocusEl={() => null}
      maxLength={64}
      onValueChange={onChange}
      onValueCommit={onUpdate}
    >
      <Heading size="lg">
        <Editable.Preview onMouseEnter={onOpen} onMouseLeave={onClose} />
        <Editable.Input border={"none"} />
      </Heading>
      <Editable.Control>
        <Editable.EditTrigger asChild>
          <IconButton
            variant="ghost"
            size="xs"
            display={open ? "inline-block" : "none"}
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
  )
}
