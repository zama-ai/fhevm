import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { SqsModule } from 'sqs'
import awsConfig from './config/aws.config.js'
import { SQSConsumer } from './infra/adapters/sqs.consumer.js'
import { SNSClient } from '@aws-sdk/client-sns'
import ethersConfig, {
  ChainId,
  EtherConfig,
  EtherConfigFactory,
  isChainId,
} from './config/ether.config.js'
import { CONTRACT_SERVICE, MESSAGE_PRODUCER } from './constants.js'
import { ContractService } from './domain/services/contract.service.js'
import { ProxyContractService } from './infra/adapters/proxy-contract.service.js'
import { AwsMessageProducer } from './infra/adapters/aws-message.producer.js'
import { MessageProducer } from './domain/services/message.producer.js'
import { DiscoverContract } from './use-cases/discover-contract.use-case.js'
import { DatabaseModule } from './infra/database/database.module.js'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig, ethersConfig],
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
    SQSConsumer,
    {
      provide: CONTRACT_SERVICE,
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        const map = config
          .get<string[]>('ether.chainIds')!
          .filter(isChainId)
          .reduce(function (acc, chainId) {
            acc.set(chainId, EtherConfigFactory.getEtherConfig(chainId))
            return acc
          }, new Map<ChainId, EtherConfig>())
        return new ProxyContractService(map)
      },
    },
    {
      provide: MESSAGE_PRODUCER,
      useClass: AwsMessageProducer,
    },
    {
      provide: DiscoverContract,
      inject: [CONTRACT_SERVICE, MESSAGE_PRODUCER],
      useFactory: (service: ContractService, producer: MessageProducer) =>
        new DiscoverContract(service, producer),
    },
  ],
})
export class AppModule {}
