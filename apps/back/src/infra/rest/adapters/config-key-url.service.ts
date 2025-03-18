import { CRS } from '#httpz/domain/entities/crs.js'
import { FHEPublicKey } from '#httpz/domain/entities/fhe-public-key.js'
import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Task, AppError, isOk } from 'utils'

@Injectable()
export class ConfigKeyUrlService extends KeyUrlService {
  private readonly logger = new Logger(ConfigKeyUrlService.name)

  constructor(private readonly config: ConfigService) {
    super()
  }

  getFHEPublicKey(): Task<FHEPublicKey[], AppError> {
    const data = this.config.get<unknown[]>('httpz.fheKeyInfo')
    this.logger.verbose(`data: ${JSON.stringify(data)}`)

    return Task.of(
      data
        ?.map(FHEPublicKey.parse)
        .filter(isOk)
        .map(r => r.unwrap()) ?? [],
    )
  }
  getCRS(): Task<CRS[], AppError> {
    const data = this.config.get<unknown[]>('httpz.crs')
    this.logger.verbose(`data: ${JSON.stringify(data)}`)

    return Task.of(
      data
        ?.map(CRS.parse)
        .filter(isOk)
        .map(r => r.unwrap()) ?? [],
    )
  }
}
