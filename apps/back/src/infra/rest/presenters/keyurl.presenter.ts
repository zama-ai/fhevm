import {
  CRS,
  FHEPublicKey,
} from '#httpz/domain/entities/value-objects/index.js'

export function KeyUrlPresenter({
  fheKeyInfo,
  crs,
}: {
  fheKeyInfo: FHEPublicKey[]
  crs: Record<string, CRS>
}) {
  return {
    fhe_key_info: fheKeyInfo
      .map(info => info.value)
      .map(({ fhePublicKey: { dataId, urls } }) => ({
        fhe_public_key: {
          data_id: dataId,
          urls,
        },
      })),
    crs: Object.fromEntries(
      Object.entries(crs).map(([key, value]) => [
        key,
        {
          data_id: value.value.dataId,
          urls: value.value.urls,
        },
      ]),
    ),
  }
}
