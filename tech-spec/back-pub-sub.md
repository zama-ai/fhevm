# back pub sub

There are several pubsub mechanisms in the back

## internal pubsub

Enables internal pubsub

- consumes events from SQS and forward them to the internal pubsub `apps/back/src/infra/sqs-consumer/sqs.consumer.ts`
- service `apps/back/src/shared/services`

## sync pubsub

the sync pubsub synchronises mutiple `back` instances

- use case
- service `apps/back/src/shared/services/sync.service.ts`

## graphql subscriptions

- all graphql subscriptions are referenced here `apps/back/src/subscriptions/domain/entities/subscription.ts` and an `asyncIterableIterator` triggers events
- subscriptions are shared accross instances using redis here `apps/back/src/subscriptions/infra/pub-sub.subscription.service.ts`
