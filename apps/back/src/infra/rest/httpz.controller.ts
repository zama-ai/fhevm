import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { InputProof } from '#httpz/use-cases/input-proof.use-case.js'
import { Body, Controller, Get, Logger, Post, UsePipes } from '@nestjs/common'
import { ZodValidationPipe } from './pipes/zod-validation.pipe.js'
import {
  InputProofRequest,
  schema as inputProofSchema,
} from './dtos/input-proof-request.dto.js'

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
    return { fhe_key_info, crs }
  }

  @Post('/input-proof')
  @UsePipes(new ZodValidationPipe(inputProofSchema))
  postInputProof(@Body() input: InputProofRequest) {
    this.logger.log('POST /input-proof')
    return this.inputProofUC.execute(input).toPromise()
  }
}
