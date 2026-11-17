// Tasks of the HCULimit allowlist test of zama-ai/planning-blockchain#1839. Run them in this order.
// One proposal is executed between two check tasks.
//
//   npx hardhat --network polygonAmoy task:allowlist-deploy-dapps
//   npx hardhat --network polygonAmoy task:allowlist-check-before-proposals
//   npx hardhat --network polygonAmoy task:allowlist-check-after-add-to-allowlist
//   npx hardhat --network polygonAmoy task:allowlist-check-after-activate
//   npx hardhat --network polygonAmoy task:allowlist-check-after-rollback
//   npx hardhat --network polygonAmoy task:allowlist-check-after-cleanup
import './1-deployDapps';
import './2-checkBeforeProposals';
import './3-checkAfterAddToAllowlist';
import './4-checkAfterActivate';
import './5-checkAfterRollback';
import './6-checkAfterCleanup';
