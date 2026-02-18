# console — fhevm Relayer

This repository hosts the **fhevm Relayer**, a Rust service that bridges fhevm (Fully Homomorphic Encryption Virtual Machine) blockchains with the Zama Gateway.

See [apps/relayer/README.md](apps/relayer/README.md) for full documentation.

## Development

All development commands (build, run, test, lint) are in `apps/relayer/`. See the [relayer README](apps/relayer/README.md) and its `Makefile`.

## Docker

```bash
docker build -f docker/relayer/Dockerfile -t console-relayer .
docker build -f docker/relayer-migrate/Dockerfile -t console-relayer-migrate .
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
