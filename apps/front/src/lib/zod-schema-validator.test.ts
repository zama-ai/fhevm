import { z } from 'zod'
import {
  toFormikValidationSchema,
  toFormikValidate,
} from './zod-schema-validator.js'

type Sut = () => {
  schema: z.ZodObject<{ name: z.ZodString; age: z.ZodNumber }>
}

describe('toFormikValidationSchema', () => {
  let makeSut: Sut
  beforeAll(() => {
    makeSut = () => {
      return {
        schema: z.object({
          name: z.string(),
          age: z.number().int('Expected number, received string'),
        }),
      }
    }
  })
  it('should pass validate without errors', async () => {
    // given
    const object = { name: 'mock', age: 32 }
    const { schema } = makeSut()
    const { validate } = toFormikValidationSchema(schema)

    // when
    const errors = await validate(object)

    // then
    expect(errors).toEqual(undefined)
  })

  it('should fail validate with error object', async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const object = { name: undefined, age: '32' } as any
    const { schema } = makeSut()
    const { validate } = toFormikValidationSchema(schema)

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const error = {} as any
    error.inner = [
      {
        path: 'name',
        message: 'Required',
      },
      {
        path: 'age',
        message: 'Expected number, received string',
      },
    ]

    await expect(validate(object)).rejects.toMatchObject(error)
  })
})

describe('toFormikValidate', () => {
  let makeSut: Sut
  beforeAll(() => {
    makeSut = () => {
      return {
        schema: z.object({
          name: z.string(),
          age: z.number().int('Expected number, received string'),
        }),
      }
    }
  })
  it('should pass validate without errors', async () => {
    // given
    const object = { name: 'mock', age: 32 }
    const { schema } = makeSut()
    const validate = toFormikValidate(schema)

    // when
    const errors = await validate(object)

    // then
    expect(errors).toEqual(undefined)
  })

  it('should fail validate with error object', async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const object = { name: undefined, age: '32' } as any
    const { schema } = makeSut()
    const validate = toFormikValidate(schema)

    const error = {
      name: 'Required',
      age: 'Expected number, received string',
    }

    const errors = await validate(object)

    expect(errors).toMatchObject(error)
  })
})
