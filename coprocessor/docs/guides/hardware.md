# Node and gateway hardware

## fhEVM validator

Validators perform all operations on ciphertext, which requires powerful machines. FHE computations benefit from multi-threading, so we recommend using [hpc7a](https://aws.amazon.com/fr/ec2/instance-types/hpc7a/) instances or equivalent, with at least 48 physical cores.

## Gateway

The gateway can run on a medium machine with 4 cores and 8 GB of RAM, such as a [t3.xlarge](https://aws.amazon.com/ec2/instance-types/t3/).

## TKMS

The TKMS needs to carry out heavy cryptographic operations on the ciphertexts. We recommend using at least a [c5.4xlarge](https://aws.amazon.com/ec2/instance-types/c5/) instance or equivalent, with at least 16 physical cores.