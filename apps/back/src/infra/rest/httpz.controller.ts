import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Controller, Get } from '@nestjs/common'

@Controller('')
export class HttpzController {
  constructor(private readonly getKeyUrlUC: GetKeyUrl) {}

  @Get('/key-url')
  getKeyUrl() {
    return this.getKeyUrlUC.execute().toPromise()
  }
}
