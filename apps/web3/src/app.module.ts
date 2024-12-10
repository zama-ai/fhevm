import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { SqsModule } from 'sqs'
import awsConfig from './config/aws.config'
import { SQSConsumer } from './infra/adapters/sqs.consumer'
import { SNSClient } from '@aws-sdk/client-sns'
import ethersConfig, { EtherProvider } from './config/ether.config'
import { VerifyContract } from './use-cases/verify-contract.use-case'
import { CONTRACT_SERVICE } from './constants'
import { EtherscanContractService } from './infra/adapters/etherscan-contract.service'
import { ContractService } from './domain/services/contract.service'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig, ethersConfig],
    }),
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
        switch (config.get<EtherProvider>('ether.provider')!) {
          case 'Etherscan':
            return new EtherscanContractService(config)
          default:
            throw new Error('invalid provider')
        }
      },
    },
    {
      provide: VerifyContract,
      inject: [CONTRACT_SERVICE],
      useFactory: (service: ContractService) => new VerifyContract(service),
    },
  ],
})
export class AppModule {}
