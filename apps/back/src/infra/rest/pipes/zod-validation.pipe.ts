import {
  ArgumentMetadata,
  BadRequestException,
  Logger,
  Paramtype,
  PipeTransform,
} from '@nestjs/common'
import { ZodSchema } from 'zod'

export class ZodValidationPipe implements PipeTransform {
  private readonly logger = new Logger(ZodValidationPipe.name)

  constructor(
    private schema: ZodSchema,
    private readonly type: Paramtype = 'body',
  ) {}

  transform(value: unknown, metadata: ArgumentMetadata) {
    this.logger.debug(
      `validating ${JSON.stringify(value)} [${JSON.stringify(metadata)}]`,
    )
    // NOTE: When using a guard that stores a value in the request,
    // it calls the validation pipe twice, the second time with the object
    // stored in the request, so it fails.
    // This monkey patch prevents this.
    if (this.type !== metadata.type) {
      this.logger.debug(
        `type mismatch: ${this.type} !== ${metadata.type} ==> ignoring validation`,
      )
      return value
    }
    const result = this.schema.safeParse(value)
    if (!result.success) {
      const message = result.error.errors
        .map(err => `${err.path}: ${err.message}`)
        .join(', ')
      this.logger.debug(`validation failed: ${message}`)
      throw new BadRequestException(`Validation failed: ${message}`)
    }

    this.logger.debug(`validation succeeded: ${JSON.stringify(result.data)}`)

    return result.data
  }
}
