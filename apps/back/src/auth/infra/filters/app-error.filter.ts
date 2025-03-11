import {
  BadRequestException,
  Catch,
  ForbiddenException,
  HttpException,
  InternalServerErrorException,
  Logger,
  NotFoundException,
  UnauthorizedException,
} from '@nestjs/common'
import { GqlExceptionFilter } from '@nestjs/graphql'
import { AppError } from 'utils'

@Catch(AppError)
export class AppErrorFilter
  implements GqlExceptionFilter<AppError, HttpException>
{
  private readonly logger = new Logger(AppErrorFilter.name)

  catch(exception: AppError) {
    this.logger.verbose(`${exception._tag}/${exception.message}`)
    switch (exception._tag) {
      case 'ForbiddenError':
        return new ForbiddenException(exception.message)

      case 'NotFoundError':
        return new NotFoundException(exception.message)

      case 'DuplicatedError':
      case 'ValidationError':
        return new BadRequestException(exception.message)

      case 'UnauthorizedError':
        return new UnauthorizedException(exception.message)

      case 'TimeoutError':
      case 'UnknownError':
      default:
        return new InternalServerErrorException(exception.message)
    }
  }
}
