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

  @Get('/keyurl')
  async getKeyUrl() {
    this.logger.log('GET /keyurl')
    const { fhe_key_info, crs } = await this.getKeyUrlUC.execute().toPromise()
    return { response: { fhe_key_info, crs } }
  }

  @Post('/input-proof')
  @UseGuards(ApiKeyGuard)
  @UsePipes(new ZodValidationPipe(inputProofSchema))
  async postInputProof(
    @CurrentApiKey() apiKey: ApiKey,
    @Body() input: InputProofRequest,
  ) {
    // const apiKey = ApiKey.parse({
    //   id: 'api_FZCIMYjgOMq7oGoqY4',
    //   dappId: 'dapp_ItjGBAb_iO9i',
    //   name: 'debug',
    // }).unwrap()
    this.logger.log('POST /input-proof')
    const response = await this.inputProofUC
      .execute(input, { apiKey })
      .toPromise()
    return { response }
  }
}
