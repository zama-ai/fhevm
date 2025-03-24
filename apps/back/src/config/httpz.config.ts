import { registerAs } from '@nestjs/config'

interface HttpzConfig {
  fheKeyInfo: {
    dataId: string
    urls: string[]
  }[]
  crs: {
    dataId: string
    urls: string[]
  }[]
}

const localHttpzConfig: HttpzConfig = {
  fheKeyInfo: [
    {
      dataId: 'fhe-public-key-data-id',
      urls: ['http://0.0.0.0:3001/publicKey.bin'],
    },
  ],
  crs: [
    {
      dataId: 'crs-data-id',
      urls: ['http://0.0.0.0:3001/crs2048.bin'],
    },
  ],
}

const prodHttpzConfig: HttpzConfig = {
  fheKeyInfo: [],
  crs: [],
}

export default registerAs('httpz', () => {
  const runMode = process.env.RUN_MODE ?? 'local'
  return runMode === 'local' ? localHttpzConfig : prodHttpzConfig
})
