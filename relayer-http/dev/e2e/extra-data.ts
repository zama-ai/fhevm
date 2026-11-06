// Prints the extraData the KMS connectors accept now (current context and epoch, from ProtocolConfig), for
// `make load` (dev/scripts/load.sh), which reads the `extraData=` line.
import { kmsExtraData } from './common';

async function main() {
  console.log(`extraData=${await kmsExtraData()}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
