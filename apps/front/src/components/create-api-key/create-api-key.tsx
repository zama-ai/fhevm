import { ApolloError } from '@apollo/client'
import { useFormik } from 'formik'
import { Fieldset, Heading, Input, HStack, Button } from '@chakra-ui/react'
import { Field } from '@/components/ui/field.js'

type OwnProps = {
  error?: ApolloError
  onCreate: (values: { name: string; description?: string }) => void
}

export function CreateApiKey({ error, onCreate }: OwnProps) {
  const formik = useFormik({
    initialValues: {
      name: '',
      description: '',
    },
    onSubmit: values => {
      const name = values.name.trim()
      const description = values.description?.trim()
      onCreate({ name, description: description ? description : undefined })
    },
  })

  return (
    <Fieldset.Root>
      <form onSubmit={formik.handleSubmit}>
        <Fieldset.Legend>
          <Heading>Create a new API KEY</Heading>
        </Fieldset.Legend>

        <Fieldset.Content>
          <HStack>
            <Field label="Name">
              <Input
                name="name"
                type="text"
                value={formik.values.name}
                onChange={formik.handleChange}
              />
            </Field>
            <Field label="Description">
              <Input
                name="description"
                type="text"
                value={formik.values.description}
                onChange={formik.handleChange}
              />
            </Field>

            <Button type="submit">Create</Button>
          </HStack>
          {error && <p>{error.message}</p>}
        </Fieldset.Content>
      </form>
    </Fieldset.Root>
  )
}
