import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { InputProof } from '#httpz/use-cases/input-proof.use-case.js'
import {
  Body,
  Controller,
  Get,
  Logger,
  Post,
  UseGuards,
  UsePipes,
} from '@nestjs/common'
import { ZodValidationPipe } from './pipes/zod-validation.pipe.js'
import {
  InputProofRequest,
  schema as inputProofSchema,
} from './dtos/input-proof-request.dto.js'
import { ApiKeyGuard } from './guards/api-key.guard.js'
import { CurrentApiKey } from './decorators/current-api-key.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'

@Controller('')
export class HttpzController {
  private readonly logger = new Logger(HttpzController.name)

  constructor(
    private readonly getKeyUrlUC: GetKeyUrl,
    private readonly inputProofUC: InputProof,
  ) {}

  @Get('/healthcheck')
  healthcheck() {
    return { response: 'ok' }
  }

  @Get('/v1/keyurl')
  async getKeyUrl() {
    this.logger.log('GET /v1/keyurl')
    const { fhe_key_info, crs } = await this.getKeyUrlUC.execute().toPromise()
    return { response: { fhe_key_info, crs } }
  }

  @Post('/v1/input-proof')
  @UseGuards(ApiKeyGuard)
  @UsePipes(new ZodValidationPipe(inputProofSchema))
  async postInputProof(
    @CurrentApiKey() apiKey: ApiKey,
    @Body() input: InputProofRequest,
  ) {
    this.logger.log('POST /v1/input-proof')
    const response = await this.inputProofUC
      .execute(input, { apiKey })
      .toPromise()
    return { response }
  }
}
