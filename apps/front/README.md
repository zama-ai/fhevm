# front

Displays the user interface

# dev

## run

```bash
# run dev in watch mode
$ pnpm start
```

## update generated graphql schemas

Generates types for all the GQL queries in the codebase.
Requires the graphql server ([apps/back](../back/READE.md)) to be running in dev mode to perform [introspection](https://graphql.org/learn/introspection/)

```bash
# update generated graphql schemas
$ pnpm generate

```

## storybook

Displays all visual components to sync with designers

```bash
# show storybook
$ pnpm storybook
```
