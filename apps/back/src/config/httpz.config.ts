import { registerAs } from '@nestjs/config'

interface HttpzConfig {
  fhe_key_info: {
    data_id: string
    urls: string[]
  }[]
  crs: {
    data_id: string
    urls: string[]
  }[]
}

const localHttpzConfig: HttpzConfig = {
  fhe_key_info: [
    {
      data_id: 'fhe-public-key-data-id',
      urls: [
        'http://0.0.0.0:9000/kms-public/kms/PUB/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee',
      ],
    },
  ],
  crs: [
    {
      data_id: 'crs-data-id',
      urls: [
        'http://0.0.0.0:9000/kms-public/kms/PUB/CRS/a5fedad3fd734a598fb67452099229445cb68447198fb56f29bb64d98953d002',
      ],
    },
  ],
}

const prodHttpzConfig: HttpzConfig = {
  fhe_key_info: [],
  crs: [],
}

export default registerAs('httpz', () => {
  const runMode = process.env.RUN_MODE ?? 'local'
  return runMode === 'local' ? localHttpzConfig : prodHttpzConfig
})
