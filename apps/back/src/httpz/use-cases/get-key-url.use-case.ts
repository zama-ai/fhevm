import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'
import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import { Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Output = { fheKeyInfo: FHEPublicKey[]; crs: CRS[] }

@Injectable()
export class GetKeyUrl implements UseCase<void, Output> {
  constructor(private readonly keyUrlService: KeyUrlService) {}

  execute(): Task<Output, AppError> {
    return Task.all<AppError, FHEPublicKey[], CRS[]>([
      this.keyUrlService.getFHEPublicKey(),
      this.keyUrlService.getCRS(),
    ]).map(([fheKeyInfo, crs]) => ({ fheKeyInfo, crs }))
  }
}
