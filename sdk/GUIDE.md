# Developing and running ESM/CJS consumer tests

## 1. To rebuild the tested package (not the test itself)

```sh
# From ./host-contracts-cleartext/v13:
npm run build
```

or

```sh
# from the fhevm/sdk root folder
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13 --build-package
```

or

```sh
# from the fhevm/sdk root folder
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13/test-consumer/<type> --build-package
```

## 2. To run the 'esm' tests (without building the tested package)

```sh
# from the fhevm/sdk root folder
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --run --ci
```

## 3. To run the 'cjs' tests (without building the tested package)

```sh
# from the fhevm/sdk root folder
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13/test-consumer/cjs --run --ci
```

## 4. To run a single consumer test file

`--test-file` selects one test file; `--run` executes it.

```sh
# from the fhevm/sdk root folder
node ./fhevm-npm/fhevm-npm.ts test-consumer \
  ./host-contracts-cleartext/v13/test-consumer/esm \
  --test-file ./test/fhe-rand.test.ts \
  --run \
  --ci
```

## 5. To rebuild the tested package and run the tests (keep the lock file)

```sh
# esm + cjs
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13 --build-package --run --ci
```

```sh
# esm only
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --build-package --run --ci
```

## 6. To regenerate the lock files, rebuild the tested package and run the tests

```sh
# esm + cjs
node ./fhevm-npm/fhevm-npm.ts test-consumer-regenerate-package-lock ./host-contracts-cleartext/v13
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13 --build-package --run --ci
```

```sh
# esm only
node ./fhevm-npm/fhevm-npm.ts test-consumer-regenerate-package-lock ./host-contracts-cleartext/v13/test-consumer/esm
node ./fhevm-npm/fhevm-npm.ts test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --build-package --run --ci
```
