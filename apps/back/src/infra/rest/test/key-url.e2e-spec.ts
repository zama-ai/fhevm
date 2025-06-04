import { IntegrationManager } from '#tests/integration.manager.js'
import { afterAll, afterEach, beforeAll, describe, expect, test } from 'vitest'
import request from 'supertest'

describe('GET /v1/keyurl', () => {
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
          .get('/v1/keyurl')
          .expect(200)
        expect(data.body, 'wrong local httpz config').toEqual({
          response: {
            fhe_key_info: [
              {
                fhe_public_key: {
                  data_id: 'fhe-public-key-data-id',
                  urls: [
                    'http://0.0.0.0:9000/kms-public/PUB/PublicKey/KEY_GEN_ID',
                  ],
                },
              },
            ],
            crs: {
              '2048': {
                data_id: 'crs-data-id',
                urls: ['http://0.0.0.0:9000/kms-public/PUB/CRS/CRS_GEN_ID'],
              },
            },
          },
        })
      })
    })
  })
})
