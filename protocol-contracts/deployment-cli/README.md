# FHEVM Deployment Automation CLI

This CLI automates the protocol deployment workflow in a single utility tool.

## Prerequisites

We recommand using [bun](https://bun.sh/) to run the CLI.
```bash
curl -fsSL https://bun.sh/install | bash
```

Then install the dependencies:
```bash
bun i
```

You can also use `npm` instead of `bun` by replacing `bun` with `npm` in the commands below. In that case,
you'll need to compile the CLI with `npm run build`, and run the javascript file directly with `node dist/index.js`.

Once done, populate the [configuration file](./deployment-state/deployment-config.yaml) with your own values.

### Commands

```bash
Usage: fhevm-deploy [options] [command]

Automated deployment CLI for the Zama FHEVM protocol

Options:
  -h, --help        display help for command

Commands:
  deploy [options]  Execute deployment steps
  status [options]  Show deployment step status from state file
  help [command]    display help for command
```

Once the deployment configuration is complete, you can run the deployment with the following command:

```bash
bun run start deploy --network testnet
```


## Tests

### Prerequisites

The test requires [anvil](https://getfoundry.sh/introduction/installation) to be installed.
They run against a forked Sepolia network on an anvil instance.
You will need to populate the `.env` file with the following values:

```bash
DAO_ADMIN_PLUGIN=<address of the DAO admin plugin contract>
DAO_ADMIN_EXECUTOR=<address of the admin executor account>
```


Running the tests requires the protocol to be deployed.

### Running the tests
```bash
bun run test

# or bun test --timeout 10000
```
