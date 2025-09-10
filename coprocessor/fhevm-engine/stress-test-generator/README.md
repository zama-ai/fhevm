# Coprocessor stress test generator
Generator for coprocessor stress tests


## Configuration

### Environment variables

 - EVGEN_SCENARIO (default: data/evgen_scenario.csv)
 - EVGEN_DB_URL (default: postgresql://postgres:postgres@127.0.0.1:5432/coprocessor)
 - ACL_CONTRACT_ADDRESS (default: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c)
 - CHAIN_ID (default: 12345)
 - API_KEY (default: a1503fb6-d79b-4e9e-826d-44cf262f3e05)
 - TENANT_ID (default 1)
 - SYNTHETIC_CHAIN_LENGTH (default: 10): used in synthetic benches (MULChain, ADDChain) for the length of each transaction
 - MIN_DECRYPTION_TYPE (default: 0): lowest type to generate (0 -> FheBool, 1 -> FheUint4, ...)
 - MAX_DECRYPTION_TYPE (default: 6 -> FheUint128)
 - OUTPUT_HANDLES_FOR_PUB_DECRYPTION (default: data/handles_for_pub_decryption)
 - OUTPUT_HANDLES_FOR_USR_DECRYPTION (default: data/handles_for_usr_decryption)

### Scenario files

The format is semicolon separated CSV with the following order:

 - Transaction type. Possible values are ERC20Transfer,
    DEXSwapRequest, DEXSwapClaim, MULChain, ADDChain, InputVerif,
    GenPubDecHandles, GenUsrDecHandles.
	
 - ERC transfer variant (only meaningful for ERC transfer using
    transactions). Possible values are Whitepaper, NoCMUX, NA,

 - Generator target. Either **Rate** or **Count**. Rate is in transactions
   generated per second. This refers to the trailing values added in
   the CSV line which supply pairs (rate/count, duration/count)
   
 - Inputs. Either **ReuseInputs** or **NewInputs** - whether to
   generate new inputs for each transaction (so encrypt, generate
   proof and send for verification) or just generate some random
   inputs at start then reuse the same inputs to avoid the delay of
   round-tripping the zkproof-worker.
 
 - Dependence. Either **Dependent** or **Independent** - whether the
   transactions will be chained together or ran independently
   (e.g. Dependent on ERC transfer means that all transfers generated
   in the scenario will transfer to the same destination wallet).
   
 - Contract address
 
 - User address
 
 - Scenario specification: unlimited sequence of pairs (float,
   integer) specifying the rate/count and duration to run the
   generator for. E.g. **1.1; 20; 3.4; 10** if using a **Rate** target
   means that the generator will issue on average 1.1 transactions per
   second for 20 seconds then 3.4 transactions per second for 10
   seconds. If the target is **Count**, then it will generate a batch
   of 1.1*20=22 transactions followed by a batch of 34 transactions.

## REST API

In addition to running the stress_generator as a CLI tool for triggering stress tests, it can also run as a standalone service on a remote machine, exposing the following APIs. The service enforces a single active job at a time, ensuring accurate performance metric collection.

   - POST job - Enqueue a new job (a set of scenarios)
   - GET /job/:id - Get status of a specific job by job_id
   - GET /status/running - Get job_id of the running job
   - GET /status/queued - List all queued jobs in the order they will run

   ### How to run
   ```bash

   # Configure all ENV variables except EVGEN_SCENARIO

   # Run as a server
   cargo run --release -- --run-server --listen-address 127.0.0.1:3030

   # Choose/prepare a json from 'data/json' files

   # Post a job
   curl -X POST http://localhost:3030/job \
     -H "Content-Type: application/json" \
     -d @./data/json/minitest_002_erc20.json

  # Get a job result
  curl -X  GET http://localhost:3030/job/0

   ```
