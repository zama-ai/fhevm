import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { SqsModule } from '@ssut/nestjs-sqs'
import awsConfig from './config/aws.config.js'
import { SQSConsumer } from './infra/adapters/sqs.consumer.js'
import { LoggerModule } from 'nestjs-pino'
import { randomUUID } from 'crypto'
import ethersConfig, {
  EtherConfig,
  EtherConfigFactory,
} from './config/ether.config.js'
import {
  CONTRACT_SERVICE,
  FHE_EVENT_REPOSITORY,
  FHE_EVENT_SERVICE,
  MS_NAME,
  PUBSUB,
} from './constants.js'
import { ContractService } from './domain/services/contract.service.js'
import { ProxyContractService } from './infra/adapters/proxy-contract.service.js'
import { SqsProducer } from './infra/adapters/sqs.producer.js'
import { DiscoverContract } from './use-cases/discover-contract.use-case.js'
import { DatabaseModule } from './infra/database/database.module.js'
import { ChainId } from './domain/entities/value-objects.js'
import { IPubSub, isOk, PubSub } from 'utils'
import fheConfig, { FheConfig, FheConfigFactory } from './config/fhe.config.js'
import { FetchFHEEvents } from './use-cases/fetch-fhe-events.use-case.js'
import { FheEventService } from './domain/services/fhe-event.service.js'
import { FheEventRepository } from './domain/services/fhe-event.repository.js'
import { PrismaFheEventRepository } from './infra/database/repositories/prisma-fhe-event.repository.js'
import { web3 } from 'messages'
import commonConfig from '#config/common.config.js'
import { ViemFheEventService } from '#infra/adapters/viem-fhe-event.service.js'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: [commonConfig, awsConfig, fheConfig, ethersConfig],
})

@Module({
  imports: [
    configModule,
    LoggerModule.forRootAsync({
      imports: [configModule],
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        return {
          pinoHttp: {
            level: config.get('common.logLevel'),
            customProps: () => ({ service: MS_NAME }),
            genReqId: request =>
              request.headers['x-correlation-id'] || randomUUID(),
            transport:
              config.get('common.nodeEnv') === 'development'
                ? {
                    target: 'pino-pretty',
                    options: {
                      singleLine: true,
                    },
                  }
                : undefined,
          },
        }
      },
    }),
    DatabaseModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'web3',
            queueUrl: config.get<string>('aws.web3.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient({
              endpoint: config.get<string>('aws.queueUrl'),
              region: config.get<string>('aws.region'),
            }),
            messageAttributeNames: ['All'],
            attributeNames: ['All'],
          },
        ],
      }),
    }),
  ],
  providers: [
    {
      provide: PUBSUB,
      useClass: PubSub,
    },
    SQSConsumer,
    {
      provide: CONTRACT_SERVICE,
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        const map = config
          .get<string[]>('ether.chainIds')!
          .map(ChainId.fromString)
          .filter(isOk)
          .map(r => r.unwrap())
          .reduce(function (acc, chainId) {
            const config = EtherConfigFactory.getEtherConfig(chainId.value)
            return config ? acc.set(chainId, config) : acc
          }, new Map<ChainId, EtherConfig>())
        return new ProxyContractService(map)
      },
    },
    SqsProducer,
    {
      provide: DiscoverContract,
      inject: [PUBSUB, CONTRACT_SERVICE],
      useFactory: (pubsub: IPubSub<web3.Web3Event>, service: ContractService) =>
        new DiscoverContract(pubsub, service),
    },
    {
      provide: FHE_EVENT_SERVICE,
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        const map = config
          .get<string[]>('fhe.chainIds')!
          .map(ChainId.fromString)
          .filter(isOk)
          .map(r => r.unwrap())
          .map(FheConfigFactory.getFheConfig)
          .filter(c => c !== null)
          .reduce(
            (map, cfg) => map.set(cfg.chainId.value, cfg),
            new Map<string, FheConfig>(),
          )
        return new ViemFheEventService(map)
      },
    },
    {
      provide: FHE_EVENT_REPOSITORY,
      useClass: PrismaFheEventRepository,
    },
    {
      provide: FetchFHEEvents,
      inject: [PUBSUB, FHE_EVENT_SERVICE, FHE_EVENT_REPOSITORY],
      useFactory: (
        pubsub: PubSub<web3.Web3Event>,
        service: FheEventService,
        repo: FheEventRepository,
      ) => new FetchFHEEvents(pubsub, service, repo),
    },
  ],
})
export class AppModule {}
