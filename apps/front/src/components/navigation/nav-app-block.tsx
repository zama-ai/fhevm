import { List, Text } from '@chakra-ui/react'
import { Circle } from 'lucide-react'

type OwnProps = {
  name: string
  color: string
  isActive?: boolean
}

export function NavAppBlock({ name, color, isActive }: OwnProps) {
  return (
    <>
      <List.Indicator
        asChild
        color={color}
        width="10px"
        opacity={0}
        _groupHover={{ opacity: 1 }}
        transition="opacity .5s"
      >
        <Circle className="circle" />
      </List.Indicator>
      <Text
        fontSize="sm"
        overflow="hidden"
        textOverflow="ellipsis"
        textWrap="nowrap"
        maxWidth="130px"
        fontWeight={isActive ? 'bold' : 'normal'}
      >
        {name.length ? name : 'New app'}
      </Text>
    </>
  )
}
