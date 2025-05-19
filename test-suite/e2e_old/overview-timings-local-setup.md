# Test locally

## Test details

- fhevmjs version:  "^0.6.0-18"
- node-tkms version: "0.9.0-rc36"

## Threshold mode

### e2e test folder (same as tests run in Sepolia)

```bash
> e2e@1.0.0 test
> hardhat test test/encryptedERC20/EncryptedERC20.ts

Compiled 9 Solidity files successfully (evm target: cancun).


  EncryptedERC20
    ✔ should mint the contract (12638ms)
Encrypting and proving in 21.7s
Verifying in 61.2s
    ✔ should transfer tokens between two users (126421ms)
Encrypting and proving in 24.3s
Verifying in 63.1s
    ✔ should not transfer tokens between two users (127910ms)
Encrypting and proving in 23.7s
Verifying in 60.7s
Encrypting and proving in 23.9s
Verifying in 60.4s
Encrypting and proving in 23.7s
Verifying in 61.7s
    ✔ should be able to transferFrom only if allowance is sufficient (343150ms)


  4 passing (10m)



  npm run test test/gateway/*                               

> e2e@1.0.0 test
> hardhat test test/gateway/createInput.ts test/gateway/decrypt.ts test/gateway/Reencrypt.fixture.ts test/gateway/reencrypt.ts



  Test input creation
Encrypting and proving in 22.1s
    1) should create an input and send it to the gateway

  TestAsyncDecrypt
    ✔ test async decrypt bool (105354ms)
    ✔ test async decrypt bool trustless (104333ms)
0n
    ✔ test async decrypt uint4 (105344ms)
    ✔ test async decrypt uint8 (104822ms)
    ✔ test async decrypt uint16 (104837ms)
    ✔ test async decrypt uint32 (105330ms)
    ✔ test async decrypt uint64 (105314ms)
    ✔ test async decrypt uint128 (104291ms)
Encrypting and proving in 21.9s
Verifying in 60.8s
    ✔ test async decrypt uint128 non-trivial (185543ms)
    ✔ test async decrypt uint256 (105310ms)
Encrypting and proving in 22.1s
Verifying in 62s
    ✔ test async decrypt uint256 non-trivial (189633ms)
    ✔ test async decrypt address (104800ms)
    ✔ test async decrypt several addresses (105307ms)
    ✔ test async decrypt mixed (104802ms)
Encrypting and proving in 22s
Verifying in 59.5s
    ✔ test async decrypt uint64 non-trivial (184801ms)
    2) test async decrypt ebytes64 trivial
Encrypting and proving in 22.2s
Verifying in 61.7s
    3) test async decrypt ebytes64 non-trivial
    4) test async decrypt ebytes128 trivial
Encrypting and proving in 23.9s
Verifying in 63.6s
```

### fhevm test folder

fhevm tag: `v0.6.0-2-test`

```bash
npx hardhat test --network localCoprocessor                                                  
Nothing to compile
No need to generate any newer typings.


  BlindAuction
    - should check Carol won the bid

  EncryptedERC20:FHEGas
    ✔ should transfer tokens between two users (63644ms)
verify_proven_ct failed with error TypeError: fetch failed
   
    1) should be able to transferFrom only if allowance is sufficient

  EncryptedERC20
    ✔ should mint the contract (13050ms)


    1) should be able to transferFrom only if allowance is sufficient

  EncryptedERC20
    ✔ should mint the contract (13050ms)
    ✔ should transfer tokens between two users (96966ms)
    ✔ should not transfer tokens between two users (102537ms)
    ✔ should be able to transferFrom only if allowance is sufficient (273399ms)

  Testing fhevmjs/fhevmjsMocked
verify_proven_ct failed with error TypeError: fetch failed
    
    2) should be able to pack up to 256 ebools
    ✔ should be unable to pack more than 256 ebools
    ✔ should be able to pack up to 32 euint64s (60450ms)
    ✔ should be unable to pack more than 32 euint64s
verify_proven_ct failed with error TypeError: fetch failed
   
    3) should be able to pack up to 2 euint128s
    ✔ should be unable to pack more than 2 euint128s
    ✔ should be able to pack up to 2048 bits but not more

  TestAsyncDecrypt
    - test async decrypt bool infinite loop
    - test async decrypt bool would fail if maxTimestamp is above 1 day
1:17:13 PM - Fulfilled decrypt on block 1212 (requestID 21)
gas paid by relayer (fulfil tx) :  502974000000000n
gas paid by user (request tx) :  154714001082998n
    ✔ test async decrypt bool (15078ms)
1:17:38 PM - Fulfilled decrypt on block 1217 (requestID 22)
    4) test async decrypt bool trustless
    - test async decrypt FAKE bool
1:17:58 PM - Fulfilled decrypt on block 1221 (requestID 23)
502848000000000n
    ✔ test async decrypt uint4 (14563ms)
    - test async decrypt FAKE uint4
1:18:19 PM - Fulfilled decrypt on block 1225 (requestID 24)
    ✔ test async decrypt uint8 (15569ms)
    - test async decrypt FAKE uint8
1:18:38 PM - Fulfilled decrypt on block 1229 (requestID 25)
    ✔ test async decrypt uint16 (15063ms)
    - test async decrypt FAKE uint16
1:18:58 PM - Fulfilled decrypt on block 1233 (requestID 26)
    ✔ test async decrypt uint32 (15569ms)
    - test async decrypt FAKE uint32
1:19:18 PM - Fulfilled decrypt on block 1237 (requestID 27)
    ✔ test async decrypt uint64 (14569ms)
1:19:38 PM - Fulfilled decrypt on block 1241 (requestID 28)
    ✔ test async decrypt uint128 (14567ms)
1:21:08 PM - Fulfilled decrypt on block 1259 (requestID 29)
    ✔ test async decrypt uint128 non-trivial (85144ms)
1:21:28 PM - Fulfilled decrypt on block 1263 (requestID 30)
    ✔ test async decrypt uint256 (14561ms)
1:23:24 PM - Fulfilled decrypt on block 1286 (requestID 31)
    ✔ test async decrypt uint256 non-trivial (110441ms)
    - test async decrypt FAKE uint64
1:23:43 PM - Fulfilled decrypt on block 1290 (requestID 32)
    ✔ test async decrypt address (14566ms)
1:24:04 PM - Fulfilled decrypt on block 1294 (requestID 33)
    ✔ test async decrypt several addresses (15569ms)
    - test async decrypt FAKE address
1:24:23 PM - Fulfilled decrypt on block 1298 (requestID 34)
    ✔ test async decrypt mixed (15070ms)
1:25:48 PM - Fulfilled decrypt on block 1315 (requestID 35)
    ✔ test async decrypt uint64 non-trivial (80532ms)
    5) test async decrypt ebytes64 trivial



    ✔ test reencrypt ebool (5672ms)
    ✔ test reencrypt euint4 (6005ms)
    ✔ test reencrypt euint8 (6100ms)
    ✔ test reencrypt euint16 (8122ms)
    ✔ test reencrypt euint32 (8126ms)
    ✔ test reencrypt euint64 (6097ms)
    ✔ test reencrypt euint128 (8120ms)
    ✔ test reencrypt eaddress (8122ms)
    ✔ test reencrypt euint256 (6111ms)

```

## Centralized mode

### e2e test folder

```bash
npm run test test/encryptedERC20/EncryptedERC20.ts                                                                               ✱ ● ✭  ~/D/d/b/f/e2e 

> e2e@1.0.0 test
> hardhat test test/encryptedERC20/EncryptedERC20.ts



  EncryptedERC20
    ✔ should mint the contract (8557ms)
Encrypting and proving in 22.6s
Verifying in 16.6s
    ✔ should transfer tokens between two users (61468ms)
Encrypting and proving in 24.6s
Verifying in 16.6s
    ✔ should not transfer tokens between two users (58309ms)
Encrypting and proving in 24.3s
Verifying in 16.9s
Encrypting and proving in 23.6s
    1) should be able to transferFrom only if allowance is sufficient



> e2e@1.0.0 test
> hardhat test test/gateway/createInput.ts test/gateway/decrypt.ts test/gateway/Reencrypt.fixture.ts test/gateway/reencrypt.ts



  Test input creation
Encrypting and proving in 22.8s
    1) should create an input and send it to the gateway

  TestAsyncDecrypt
    ✔ test async decrypt bool (24593ms)
    ✔ test async decrypt bool trustless (25098ms)
0n
    ✔ test async decrypt uint4 (25099ms)
    ✔ test async decrypt uint8 (25102ms)
    ✔ test async decrypt uint16 (24585ms)
    ✔ test async decrypt uint32 (25080ms)
    ✔ test async decrypt uint64 (25587ms)
    ✔ test async decrypt uint128 (24587ms)
Encrypting and proving in 21.7s
Verifying in 16.3s
    ✔ test async decrypt uint128 non-trivial (139810ms)
    ✔ test async decrypt uint256 (104820ms)
Encrypting and proving in 21.8s
Verifying in 18.3s
    ✔ test async decrypt uint256 non-trivial (145851ms)
    ✔ test async decrypt address (104828ms)
    ✔ test async decrypt several addresses (104834ms)
    ✔ test async decrypt mixed (105328ms)
Encrypting and proving in 21.4s
Verifying in 17s
    ✔ test async decrypt uint64 non-trivial (139691ms)
    2) test async decrypt ebytes64 trivial
Encrypting and proving in 22s
Verifying in 17.2s
    3) test async decrypt ebytes64 non-trivial
    4) test async decrypt ebytes128 trivial
Encrypting and proving in 22.9s
Verifying in 18.3s
    5) test async decrypt ebytes128 non-trivial
    ✔ test async decrypt ebytes256 trivial (104800ms)
Encrypting and proving in 23.3s
Verifying in 19.5s
    ✔ test async decrypt ebytes256 non-trivial (145045ms)
    ✔ test async decrypt ebytes256 non-trivial with snapshot [skip-on-coverage]
Encrypting and proving in 23.6s
Verifying in 20.2s
    6) test async decrypt mixed with ebytes256
Encrypting and proving in 23.4s
Verifying in 20s
    ✔ test async decrypt ebytes256 non-trivial trustless (145197ms)
Encrypting and proving in 23.6s
Verifying in 19.7s
    7) test async decrypt mixed with ebytes256 trustless

  Test reencrypt
    ✔ should reencrypt a bool value (4866ms)
    ✔ should reencrypt a 4bits value (4051ms)
    ✔ should reencrypt a 8bits value (4060ms)
    ✔ should reencrypt a 16bits value (4050ms)
    ✔ should reencrypt a 32bits value (4061ms)
    ✔ should reencrypt a 64bits value (5092ms)
    ✔ should reencrypt a 128bits value (4059ms)
    ✔ should reencrypt a 256bits value (4064ms)


```

### fhevm test repository

fhevm tag: `v0.6.0-2`

```bash

 npx hardhat test  --network ocalCoprocessor

Nothing to compile
No need to generate any newer typings.


  BlindAuction
    - should check Carol won the bid

  EncryptedERC20:FHEGas
    ✔ should transfer tokens between two users (64304ms)
verify_proven_ct failed with error TypeError: fetch failed
    at Object.fetch (node:internal/deps/undici/undici:11730:11)
    at async Object.encrypt (/home/ldemir/Documents/dev/blockchain/fhevm-L1-demo/coprocessor/work_dir/fhevm/node_modules/fhevmjs/lib/node.cjs:900:34)
    at async Context.<anonymous> (/home/ldemir/Documents/dev/blockchain/fhevm-L1-demo/coprocessor/work_dir/fhevm/test/encryptedERC20/EncryptedERC20.FHEGas.ts:61:38) {
    1) should be able to transferFrom only if allowance is sufficient

  EncryptedERC20
    ✔ should mint the contract (9637ms)
    ✔ should transfer tokens between two users (75161ms)
    ✔ should not transfer tokens between two users (75512ms)
verify_proven_ct failed with error TypeError: fetch failed

    2) should be able to transferFrom only if allowance is sufficient

  Testing fhevmjs/fhevmjsMocked
    3) should be able to pack up to 256 ebools
    ✔ should be unable to pack more than 256 ebools
verify_proven_ct failed with error TypeError: fetch failed

    4) should be able to pack up to 32 euint64s
    ✔ should be unable to pack more than 32 euint64s
    ✔ should be able to pack up to 2 euint128s (62297ms)
    ✔ should be unable to pack more than 2 euint128s
    ✔ should be able to pack up to 2048 bits but not more

  TestAsyncDecrypt
    - test async decrypt bool infinite loop
    - test async decrypt bool would fail if maxTimestamp is above 1 day
1:55:17 PM - Fulfilled decrypt on block 2273 (requestID 28)
gas paid by relayer (fulfil tx) :  457228800000000n
gas paid by user (request tx) :  154714001082998n
    ✔ test async decrypt bool (15584ms)
1:55:41 PM - Fulfilled decrypt on block 2278 (requestID 29)
    5) test async decrypt bool trustless
    - test async decrypt FAKE bool
1:56:01 PM - Fulfilled decrypt on block 2282 (requestID 30)
457189200000000n
    ✔ test async decrypt uint4 (15568ms)
    - test async decrypt FAKE uint4
1:56:16 PM - Fulfilled decrypt on block 2285 (requestID 31)
    ✔ test async decrypt uint8 (9549ms)
    - test async decrypt FAKE uint8
1:56:36 PM - Fulfilled decrypt on block 2289 (requestID 32)
    ✔ test async decrypt uint16 (15071ms)
    - test async decrypt FAKE uint16
1:56:56 PM - Fulfilled decrypt on block 2293 (requestID 33)
    ✔ test async decrypt uint32 (14573ms)
    - test async decrypt FAKE uint32
1:57:17 PM - Fulfilled decrypt on block 2297 (requestID 34)
    ✔ test async decrypt uint64 (14571ms)
1:57:37 PM - Fulfilled decrypt on block 2301 (requestID 35)
    ✔ test async decrypt uint128 (15070ms)
1:58:52 PM - Fulfilled decrypt on block 2316 (requestID 36)
    ✔ test async decrypt uint128 non-trivial (69982ms)
1:59:11 PM - Fulfilled decrypt on block 2320 (requestID 37)
    ✔ test async decrypt uint256 (14568ms)
2:00:21 PM - Fulfilled decrypt on block 2334 (requestID 38)
    ✔ test async decrypt uint256 non-trivial (65300ms)
    - test async decrypt FAKE uint64
2:00:41 PM - Fulfilled decrypt on block 2338 (requestID 39)
    ✔ test async decrypt address (15069ms)
2:01:01 PM - Fulfilled decrypt on block 2342 (requestID 40)
    ✔ test async decrypt several addresses (14569ms)
    - test async decrypt FAKE address
2:01:22 PM - Fulfilled decrypt on block 2346 (requestID 41)
    ✔ test async decrypt mixed (14073ms)
2:02:36 PM - Fulfilled decrypt on block 2361 (requestID 42)
    ✔ test async decrypt uint64 non-trivial (70057ms)

```
