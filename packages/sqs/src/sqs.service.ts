import { DiscoveryService } from '@golevelup/nestjs-discovery';
import {
  Inject,
  Injectable,
  Logger,
  LoggerService,
  OnModuleDestroy,
  OnModuleInit,
} from '@nestjs/common';
import {
  SqsConsumerEventHandlerMeta,
  SqsConsumerMapValues,
  SqsMessageHanlderMeta,
  SqsOptions,
} from './sqs.types.js';
import { StopOptions } from 'sqs-consumer';
import {
  SQS_CONSUMER_EVENT_HANDLER,
  SQS_CONSUMER_METHOD,
  SQS_OPTIONS,
} from './sqs.constants.js';
import { Consumer } from 'sqs-consumer';
import { Producer } from './producer.js';

@Injectable()
export class SqsService implements OnModuleInit, OnModuleDestroy {
  public readonly consumers = new Map<string, SqsConsumerMapValues>();
  public readonly producers = new Map<string, Producer>();

  private readonly logger: LoggerService;
  private readonly globalStopOptions: StopOptions;

  public constructor(
    @Inject(SQS_OPTIONS) public readonly options: SqsOptions,
    private readonly discover: DiscoveryService,
  ) {
    this.logger = this.options.logger ?? new Logger(SqsService.name);
    this.globalStopOptions = this.options.globalStopOptions ?? {};
  }

  async onModuleInit() {
    const messageHandlers =
      await this.discover.providerMethodsWithMetaAtKey<SqsMessageHanlderMeta>(
        SQS_CONSUMER_METHOD,
      );
    const eventHandlers =
      await this.discover.providerMethodsWithMetaAtKey<SqsConsumerEventHandlerMeta>(
        SQS_CONSUMER_EVENT_HANDLER,
      );

    this.options.consumers?.forEach(({ name, stopOptions, ...options }) => {
      if (this.consumers.has(name)) {
        throw new Error(`Consumer already exists: ${name}`);
      }

      const metadata = messageHandlers.find(({ meta }) => meta.name === name);
      if (!metadata) {
        this.logger.warn(`No metadata found for: ${name}`);
        return;
      }

      const isBatchHandler = metadata.meta.batch ?? false;
      const handler = metadata.discoveredMethod.handler.bind(
        metadata.discoveredMethod.parentClass.instance,
      );
      const consumer = Consumer.create({
        ...options,
        ...(isBatchHandler
          ? { handleMessageBatch: handler }
          : { handleMessage: handler }),
      });

      const eventsMetadata = eventHandlers.filter(
        ({ meta }) => meta.name === name,
      );
      for (const eventMetadata of eventsMetadata) {
        if (eventMetadata) {
          consumer.addListener(
            eventMetadata.meta.eventName,
            eventMetadata.discoveredMethod.handler.bind(
              metadata.discoveredMethod.parentClass.instance,
            ),
          );
        }
      }
      this.consumers.set(name, {
        instance: consumer,
        stopOptions: stopOptions ?? this.globalStopOptions,
      });
    });

    this.options.producers?.forEach(({ name, ...options }) => {
      if (this.producers.has(name)) {
        throw new Error(`Producer already exists: ${name}`);
      }

      this.producers.set(name, Producer.create(options));
    });

    for (const consumer of this.consumers.values()) {
      consumer.instance.start();
    }
  }

  onModuleDestroy() {
    for (const consumer of this.consumers.values()) {
      consumer.instance.stop(consumer.stopOptions);
    }
  }
}
