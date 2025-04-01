import { BadRequestException, Logger, PipeTransform } from '@nestjs/common'
import { ZodSchema } from 'zod'

export class ZodValidationPipe implements PipeTransform {
  private readonly logger = new Logger(ZodValidationPipe.name)

  constructor(private schema: ZodSchema) {}
  transform(value: unknown) {
    const result = this.schema.safeParse(value)
    if (!result.success) {
      const message = result.error.errors
        .map(err => `${err.path}: ${err.message}`)
        .join(', ')
      this.logger.debug(`validation failed: ${message}`)
      throw new BadRequestException(`Validation failed: ${message}`)
    }

    return result.data
  }
}
