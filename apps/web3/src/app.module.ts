import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { SqsModule } from 'sqs'
import awsConfig from './config/aws.config.js'
import { SQSConsumer } from './infra/adapters/sqs.consumer.js'
import { SNSClient } from '@aws-sdk/client-sns'
import ethersConfig, {
  EtherConfig,
  EtherConfigFactory,
} from './config/ether.config.js'
import {
  CONTRACT_SERVICE,
  FHE_EVENT_REPOSITORY,
  FHE_EVENT_SERVICE,
  MESSAGE_PRODUCER,
  PUBSUB,
} from './constants.js'
import { ContractService } from './domain/services/contract.service.js'
import { ProxyContractService } from './infra/adapters/proxy-contract.service.js'
import { SnsProducer } from './infra/adapters/sns.producer.js'
import { MessageProducer } from './domain/services/message.producer.js'
import { DiscoverContract } from './use-cases/discover-contract.use-case.js'
import { DatabaseModule } from './infra/database/database.module.js'
import { ChainId } from './domain/entities/value-objects.js'
import { isOk, PubSub } from 'utils'
import fheConfig, { FheConfig, FheConfigFactory } from './config/fhe.config.js'
import { EthersFheEventService } from './infra/adapters/ethers-fhe-event.service.js'
import { FetchFHEEvents } from './use-cases/fetch-fhe-events.use-case.js'
import { FheEventService } from './domain/services/fhe-event.service.js'
import { FheEventRepository } from './domain/services/fhe-event.repository.js'
import { PrismaFheEventRepository } from './infra/database/repositories/prisma-fhe-event.repository.js'
import { web3 } from 'messages'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig, fheConfig, ethersConfig],
    }),
    DatabaseModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'web3',
            queueUrl: config.get<string>('aws.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient({
              endpoint: config.get<string>('aws.queueUrl'),
              region: config.get<string>('aws.region'),
            }),
            messageAttributeNames: ['All'],
            attributeNames: ['All'],
          },
        ],
        producers: [
          {
            name: 'console',
            topicArn: config.get<string>('aws.topicArn')!,
            sns: new SNSClient({
              endpoint: config.get<string>('aws.endpoint'),
              region: config.get<string>('aws.region'),
            }),
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
    {
      provide: MESSAGE_PRODUCER,
      useClass: SnsProducer,
    },
    {
      provide: DiscoverContract,
      inject: [CONTRACT_SERVICE, MESSAGE_PRODUCER],
      useFactory: (service: ContractService, producer: MessageProducer) =>
        new DiscoverContract(service, producer),
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
        return new EthersFheEventService(map)
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
