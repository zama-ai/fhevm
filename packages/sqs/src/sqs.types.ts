import { LoggerService, ModuleMetadata, Type } from '@nestjs/common';
import type { Consumer, ConsumerOptions, StopOptions } from 'sqs-consumer';

export type SqsConsumerOptions = Omit<
  ConsumerOptions,
  'handleMessage' | 'handleMessageBatch'
> & {
  name: string;
  stopOptions?: StopOptions;
};

export type SqsConsumerMapValues = {
  instance: Consumer;
  stopOptions: StopOptions;
};

export interface SqsOptions {
  consumers: SqsConsumerOptions[];
  logger?: LoggerService;
  globalStopOptions?: StopOptions;
}

export interface SqsModuleOptionsFactory {
  createOptions(): Promise<SqsOptions> | SqsOptions;
}

export interface SqsModuleAsyncOptions extends Pick<ModuleMetadata, 'imports'> {
  inject?: any[];
  useFactory?: (...args: any[]) => Promise<SqsOptions> | SqsOptions;
  useExisting?: Type<SqsModuleOptionsFactory>;
  useClass?: Type<SqsModuleOptionsFactory>;
}

export interface SqsMessageHanlderMeta {
  name: string;
  batch?: boolean;
}

export interface SqsConsumerEventHandlerMeta {
  name: string;
  eventName: string;
}
