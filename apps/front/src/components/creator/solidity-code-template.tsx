import { ClipboardButton, ClipboardRoot } from '#components/ui/clipboard.js'
import { Box } from '@chakra-ui/react'
import { Highlight, themes } from 'prism-react-renderer'

const codeBlock = `// this is a sample code
// let's ask etienne or clement
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice A simple contract that maintains a single state variable 'value'
/// @dev This contract provides functionality to increment the 'value' and read its current value
contract Counter {
    /// @notice State variable to keep track of the count
    /// @dev Stored as a uint32 to save gas
    uint32 value;

    /// @notice Increases the value by 1 each time this function is called
    function increment() public {
        value += 1;
    }

    /// @notice Returns the current value of the counter
    /// @return The current value as a uint32
    function currentValue() public view returns (uint32) {
        return value;
    }
}
`

export function SolidityCodeTemplate() {
  return (
    <Highlight theme={themes.oneLight} code={codeBlock} language="tsx">
      {({ style, tokens, getLineProps, getTokenProps }) => (
        <Box
          style={style}
          rounded="md"
          as="pre"
          p="3"
          overflow="scroll"
          position="relative"
        >
          <ClipboardRoot
            value={codeBlock}
            position="absolute"
            style={{ right: 10 }}
          >
            <ClipboardButton />
          </ClipboardRoot>
          {tokens.map((line, i) => (
            <Box
              display="block"
              fontSize="xs"
              as="div"
              key={i}
              {...getLineProps({ line })}
            >
              {' '}
              <Box display="inline-block" color="gray.300">
                {i + 1}
              </Box>
              <Box display="inline-block" pl="2">
                {line.map((token, key) => (
                  <span key={key} {...getTokenProps({ token })} />
                ))}
              </Box>
            </Box>
          ))}
        </Box>
      )}
    </Highlight>
  )
}
