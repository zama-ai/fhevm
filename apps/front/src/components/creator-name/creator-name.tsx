import { Box, Fieldset, Grid, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'
import { Field } from '@/components/ui/field'
import { SpinnerButton } from '@/components/ui/spinner-button'
import { Highlight, themes } from 'prism-react-renderer'

type OwnProps = {
  onSubmit: (values: { name: string }) => void
  loading: boolean
  errorMessage?: string
}

const codeBlock = `// SPDX-License-Identifier: BSD-3-Clause-Clear
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

function CodeHighlight() {
  return (
    <Highlight theme={themes.oneLight} code={codeBlock} language="tsx">
      {({ style, tokens, getLineProps, getTokenProps }) => (
        <Box style={style} rounded="md" as="pre" p="3" overflow="scroll">
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

function Tutorial() {
  return (
    <>
      <iframe
        width="100%"
        height="200"
        src="https://www.youtube.com/embed/1FtbyHZwNX4?si=2aBiv5N58ffKXD_L"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      ></iframe>
      <Text fontSize="xs">
        <b>Tutorial:</b> Use this solidity code in Remix and deploy it on
        Sepolia.
      </Text>
    </>
  )
}

export function CreatorName({ onSubmit, loading, errorMessage }: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
    },
    onSubmit: values => {
      onSubmit(values)
    },
  })
  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Stack gap="5">
          <Fieldset.Content>
            <Field label="dApp name">
              <Input
                disabled={loading}
                name="name"
                type="text"
                placeholder="My first dApp"
                onChange={formik.handleChange}
                value={formik.values.name}
                required
              />
            </Field>
          </Fieldset.Content>
          {errorMessage && (
            <Text color="red.500" fontSize="sm">
              {errorMessage}
            </Text>
          )}
          <Text fontSize="sm" fontWeight="medium" mb={0} pb={0}>
            Solidity Code
          </Text>
          <Grid templateColumns="repeat(2, 1fr)" gap="6">
            <CodeHighlight />
            <Box>
              <Tutorial />
            </Box>
          </Grid>

          <Box display="flex" justifyContent="flex-end">
            <SpinnerButton
              loading={loading}
              loadingText="Saving..."
              type="submit"
              alignSelf="flex-start"
              disabled={loading}
            >
              Next step
            </SpinnerButton>
          </Box>
        </Stack>
      </form>
    </Fieldset.Root>
  )
}
