import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import {
  KEY_URL_SERVICE,
  KeyUrlService,
} from '#httpz/domain/service/key-url.service.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Output = { fheKeyInfo: FHEPublicKey[]; crs: Record<string, CRS> }

@Injectable()
export class GetKeyUrl implements UseCase<void, Output> {
  constructor(
    @Inject(KEY_URL_SERVICE) private readonly keyUrlService: KeyUrlService,
  ) {}

  execute = (): Task<Output, AppError> => {
    return Task.all<AppError, FHEPublicKey[], Record<string, CRS>>([
      this.keyUrlService.getFHEPublicKey(),
      this.keyUrlService.getCRS(),
    ]).map(([fheKeyInfo, crs]) => ({ fheKeyInfo, crs }))
  }
}
