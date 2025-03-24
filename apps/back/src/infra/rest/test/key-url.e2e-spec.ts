import { IntegrationManager } from '#tests/integration.manager.js'
import { afterAll, afterEach, beforeAll, describe, expect, test } from 'vitest'
import request from 'supertest'

describe('GET /key-url', () => {
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
          .get('/key-url')
          .expect(200)
        expect(data.body, 'wrong local httpz config').toEqual({
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
        })
      })
    })
  })
})
