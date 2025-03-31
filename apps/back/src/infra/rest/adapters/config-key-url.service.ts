import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Task, AppError, isOk } from 'utils'

@Injectable()
export class ConfigKeyUrlService implements KeyUrlService {
  private readonly logger = new Logger(ConfigKeyUrlService.name)

  constructor(private readonly config: ConfigService) {}

  getFHEPublicKey(): Task<FHEPublicKey[], AppError> {
    const data = this.config.get<unknown[]>('httpz.fhe_key_info')
    this.logger.verbose(`data: ${JSON.stringify(data)}`)

    return Task.of(
      data
        ?.map(FHEPublicKey.parse)
        .filter(isOk)
        .map(r => r.unwrap()) ?? [],
    )
  }
  getCRS(): Task<Record<string, CRS>, AppError> {
    const data = this.config.get<Record<string, unknown>>('httpz.crs') ?? {}
    this.logger.verbose(`data: ${JSON.stringify(data)}`)

    return Task.of(
      Object.entries(data).reduce(
        (map, [key, value]) => {
          const crs = CRS.parse(value)
          return isOk(crs) ? { ...map, [key]: crs.unwrap() } : map
        },
        {} as Record<string, CRS>,
      ),
    )
  }
}
