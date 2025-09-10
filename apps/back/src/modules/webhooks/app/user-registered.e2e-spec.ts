import { HttpService } from '@nestjs/axios'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import type { MockInstance } from 'vitest'
import { faker } from '@faker-js/faker'
import { of } from 'rxjs'
import type * as tyk from './tyk.types.js'
import { AxiosResponse } from 'axios'
import request from 'supertest'
import { AppModule, configModule } from '#app.module.js'
import { ConfigModule } from '@nestjs/config'
import configuration from '#config/configuration.js'
import { WebhookPayload } from './webhooks.types.js'

describe('UserRegistered', () => {
  let app: INestApplication
  let httpService: HttpService

  beforeEach(async () => {
    const moduleFixture = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
            configuration,
            () => ({
              tyk: {
                endpoint: 'http://localhost:8080',
                apiKey: 'test',
              },
            }),
          ],
        }),
      )
      .overrideProvider(HttpService)
      .useValue({
        post: vi.fn(),
        put: vi.fn(),
      })
      .compile()

    app = moduleFixture.createNestApplication()
    await app.init()

    httpService = moduleFixture.get<HttpService>(HttpService)
  })

  afterEach(async () => {
    if (app) {
      await app.close()
    }
  })

  describe('given a user registered event', () => {
    let event: Extract<WebhookPayload, { Event: 'UserRegistered' }>

    beforeEach(() => {
      event = {
        Event: 'UserRegistered',
        Message: {
          ID: faker.number.int({ min: 1 }),
          Email: faker.internet.email(),
          First: faker.person.firstName(),
          Last: faker.person.lastName(),
          OrgID: faker.number.int({ min: 2 }),
          Provider: faker.string.alphanumeric(10),
          CreatedAt: faker.date.past().toISOString(),
          CustomAttributes: [],
        },
      }
    })

    describe('when the user belongs to the default organization', () => {
      let newOrg: tyk.NewOrganizationResponse
      let httpPostSpy: MockInstance
      let httpPutSpy: MockInstance

      beforeEach(() => {
        event.Message.OrgID = 1

        // POST /organisations response
        newOrg = {
          ID: faker.number.int({ min: 1 }),
          Name: faker.string.alphanumeric(10),
          Teams: [
            {
              ID: faker.number.int({ min: 1 }),
              Name: faker.string.alphanumeric(10),
              Default: true,
            },
          ],
        }

        // POST /organisations
        httpPostSpy = vi.spyOn(httpService, 'post').mockImplementation(() =>
          of({
            data: newOrg,
            status: 201,
            statusText: 'Created',
          } as AxiosResponse),
        )

        httpPutSpy = vi.spyOn(httpService, 'put').mockImplementation(() =>
          of({
            data: newOrg,
            status: 200,
            statusText: 'OK',
          } as AxiosResponse),
        )
      })

      test('should create a new organisation', async () => {
        await request(app.getHttpServer())
          .post('/webhooks')
          .send(event)
          .expect(202)

        expect(httpPostSpy).toHaveBeenCalledExactlyOnceWith('/organisations', {
          Name: `${event.Message.First} ${event.Message.Last}'s Org`,
        })
      })

      test('should assign the user to the new organisation', async () => {
        await request(app.getHttpServer())
          .post('/webhooks')
          .send(event)
          .expect(202)

        expect(httpPutSpy).toHaveBeenCalledExactlyOnceWith(
          `/users/${event.Message.ID}`,
          {
            OrganisationID: newOrg.ID,
            Teams: [newOrg.Teams[0].ID],
          },
        )
      })
    })

    describe("when the user doesn't belong to the default organization", () => {
      let httpPostSpy: MockInstance
      let httpPutSpy: MockInstance

      beforeEach(() => {
        httpPostSpy = vi.spyOn(httpService, 'post')
        httpPutSpy = vi.spyOn(httpService, 'put')
      })

      test('should not create a new organisation', async () => {
        await request(app.getHttpServer())
          .post('/webhooks')
          .send(event)
          .expect(202)

        expect(httpPostSpy).not.toHaveBeenCalled()
      })

      test('should not assign the user to the new organisation', async () => {
        await request(app.getHttpServer())
          .post('/webhooks')
          .send(event)
          .expect(202)

        expect(httpPutSpy).not.toHaveBeenCalled()
      })
    })
  })
})
