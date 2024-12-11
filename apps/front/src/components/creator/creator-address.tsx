import { Box, Fieldset, Input, Stack, Text } from '@chakra-ui/react'
import { useFormik } from 'formik'

import { Field } from '@/components/ui/field'
import { SpinnerButton } from '@/components/ui/spinner-button'

import { Alert } from '../ui/alert'

type OwnProps = {
  onSubmit: (values: { name: string }) => void
  loading: boolean
  errorMessage?: string
}

function Tutorial() {
  return (
    <Box>
      <iframe
        width="100%"
        height="200"
        src="https://www.youtube.com/embed/1FtbyHZwNX4?si=2aBiv5N58ffKXD_L"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      ></iframe>
      <Text fontSize="xs" my="1">
        <b>Tutorial:</b> Use this solidity code in Remix and deploy it on
        Sepolia.
      </Text>
      <Alert status="warning" title="Some warning to be redacted" my="5">
        This code is signed, don't share it with others. Ask Roger to rephrase
        this warning.
      </Alert>
    </Box>
  )
}

export function CreatorAddress({ onSubmit, loading, errorMessage }: OwnProps) {
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
          <Fieldset.Content w={{ base: 'full', md: '1/2' }}>
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

          <Tutorial />
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
