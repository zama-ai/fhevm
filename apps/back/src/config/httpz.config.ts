import { registerAs } from '@nestjs/config'

interface HttpzConfig {
  fhe_key_info: {
    fhe_public_key: {
      data_id: string
      urls: string[]
    }
  }[]
  crs: Record<
    string,
    {
      data_id: string
      urls: string[]
    }
  >
}

const localHttpzConfig: HttpzConfig = {
  fhe_key_info: [
    {
      fhe_public_key: {
        data_id: 'fhe-public-key-data-id',
        urls: (process.env.KEY_URLS || '')
          .split(',')
          .map(url => url.trim())
          .filter(url => url.length > 0),
      },
    },
  ],
  crs: {
    '2048': {
      data_id: 'crs-data-id',
      urls: (process.env.CRS_URLS || '')
        .split(',')
        .map(url => url.trim())
        .filter(url => url.length > 0),
    },
  },
}

const prodHttpzConfig: HttpzConfig = {
  fhe_key_info: [],
  crs: {},
}

export default registerAs('httpz', () => {
  const runMode = process.env.RUN_MODE ?? 'local'
  return runMode === 'local' ? localHttpzConfig : prodHttpzConfig
})
