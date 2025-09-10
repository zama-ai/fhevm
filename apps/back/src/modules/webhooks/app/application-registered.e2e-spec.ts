import { AppModule, configModule } from '#app.module.js'
import configuration from '#config/configuration.js'
import { faker } from '@faker-js/faker'
import { HttpService } from '@nestjs/axios'
import { INestApplication } from '@nestjs/common'
import { ConfigModule } from '@nestjs/config'
import { Test } from '@nestjs/testing'
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  MockInstance,
  test,
  vi,
} from 'vitest'
import { WebhookPayload } from './webhooks.types.js'
import type * as tyk from './tyk.types.js'
import { of } from 'rxjs'
import { Axios, AxiosResponse } from 'axios'
import request from 'supertest'

describe('ApplicationRegistered', () => {
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
        get: vi.fn(),
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

  describe('given an application registered event', () => {
    let event: Extract<WebhookPayload, { Event: 'ApplicationRegistered' }>

    beforeEach(() => {
      event = {
        Event: 'ApplicationRegistered',
        Message: {
          ID: faker.number.int({ min: 2 }),
          Name: faker.string.alphanumeric(10),
          UserID: faker.number.int({ min: 1 }),
          CreatedAt: faker.date.past().toISOString(),
        },
      }
    })

    describe('when there is a default plan', () => {
      let plans: tyk.PlanDetail[]
      let product: tyk.Product
      let httpGetSpy: MockInstance
      let httpPutSpy: MockInstance

      beforeEach(async () => {
        plans = [
          {
            ID: faker.number.int({ min: 1 }),
            Name: faker.string.alphanumeric(10),
            AutoApproveAccessRequests: true,
            Description: faker.lorem.sentence(),
            DisplayName: faker.lorem.words(3),
            MetaData: { default: 'false' },
          },
          {
            ID: faker.number.int({ min: 1 }),
            Name: faker.string.alphanumeric(10),
            AutoApproveAccessRequests: true,
            Description: faker.lorem.sentence(),
            DisplayName: faker.lorem.words(3),
            MetaData: { default: 'true' },
          },
        ]
        product = {
          ID: faker.number.int({ min: 1 }),
          Name: faker.string.alphanumeric(10),
          DisplayName: faker.lorem.words(3),
          Feature: true,
        }

        httpGetSpy = vi.spyOn(httpService, 'get').mockImplementation(url => {
          switch (url) {
            case 'products':
              return of({
                data: [product],
                status: 200,
                statusText: 'OK',
              } as AxiosResponse)
            case 'plans':
              return of({
                data: plans,
                status: 200,
                statusText: 'OK',
              } as AxiosResponse)
            default:
              // GET plans/:planID
              const planID = url.split('/').pop()
              const plan = plans.find(p => p.ID === Number(planID))
              return plan
                ? of({
                    data: plan,
                    status: 200,
                    statusText: 'OK',
                  } as AxiosResponse)
                : of({
                    data: [],
                    status: 404,
                    statusText: 'Not Found',
                  } as AxiosResponse)
          }
        })

        httpPutSpy = vi.spyOn(httpService, 'put').mockImplementation(() =>
          of({
            data: {
              message: 'Action provision: Executed successfully',
              status: 'ok',
            },
            status: 200,
            statusText: 'OK',
          } as AxiosResponse),
        )
        await request(app.getHttpServer())
          .post('/webhooks')
          .send(event)
          .expect(202)
      })

      test('then it should fetch all plans', async () => {
        expect(httpGetSpy).toHaveBeenCalledWith('plans')
      })

      test('then it should fetch each plan', async () => {
        expect(httpGetSpy).toHaveBeenCalledWith(`plans/${plans[0].ID}`)
        expect(httpGetSpy).toHaveBeenCalledWith(`plans/${plans[1].ID}`)
      })

      test('then should create a new access request', async () => {
        expect(httpPutSpy).toHaveBeenCalledWith(
          `/apps/${event.Message.ID}/provision`,
          {
            PlanID: plans[1].ID,
            ProductIDs: [product.ID],
          },
        )
      })
    })
  })
})
