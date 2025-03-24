import { IntegrationManager } from '#tests/integration.manager.js'
import { afterAll, afterEach, beforeAll, describe, expect, test } from 'vitest'
import request from 'supertest'

describe('GET /keyurl', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30000)

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a user', () => {
    describe('when requesting the key url', () => {
      test('then it should return the public info', async () => {
        const data = await request(manager.httpServer)
          .get('/keyurl')
          .expect(200)
        expect(data.body, 'wrong local httpz config').toEqual({
          response: {
            fhe_key_info: [
              {
                fhe_public_key: {
                  data_id: 'fhe-public-key-data-id',
                  urls: [
                    'http://0.0.0.0:9000/kms-public/kms/PUB/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee',
                  ],
                },
              },
            ],
            crs: {
              '2048': {
                data_id: 'crs-data-id',
                urls: [
                  'http://0.0.0.0:9000/kms-public/kms/PUB/CRS/a5fedad3fd734a598fb67452099229445cb68447198fb56f29bb64d98953d002',
                ],
              },
            },
          },
        })
      })
    })
  })
})
