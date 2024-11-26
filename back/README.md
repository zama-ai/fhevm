# Description

The Api-gateway provide data to to the frontend, auth users, stores users & apps data.

## Compile and run the project

```bash
# development
$ npm run start

# watch mode
$ npm run start:dev

# production mode
$ npm run start:prod
```

## Manage the database
```bash
# apply prisma migrations
$ npx prisma migrate dev

# edit the database in prisma studio
$ npx prisma studio

```


## Run tests

```bash
# unit tests
$ npm run test

# e2e tests
$ npm run test:e2e

# test coverage
$ npm run test:cov
```

## Resources

Check out a few resources that may come in handy when working with NestJS:

- [NestJS Documentation](https://docs.nestjs.com) to learn more about the framework.
- [Prisma Documentation](https://www.prisma.io/docs/orm) to learn more about the orm.
- [Apollo Graphql Server Documentation](https://www.apollographql.com/docs/apollo-server/schema/schema)
- Visualize your application graph and interact with the NestJS application in real-time using [NestJS Devtools](https://devtools.nestjs.com).
