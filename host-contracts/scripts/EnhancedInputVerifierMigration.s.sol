// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script} from "forge-std/Script.sol";
import {EnhancedInputVerifier} from "../contracts/EnhancedInputVerifier.sol";
import {InputVerifier} from "../contracts/InputVerifier.sol";
import {console} from "forge-std/console.sol";

/**
 * @title EnhancedInputVerifierMigration
 * @notice Migration script from InputVerifier to EnhancedInputVerifier
 * @dev This script handles the secure migration with validation
 */
contract EnhancedInputVerifierMigration is Script {
    // Configuration
    address public verifyingContractSource;
    uint256 public chainIDSource;

    // Events
    event MigrationStarted(address oldVerifier, address newVerifier);
    event MigrationCompleted(address newVerifier, uint256 threshold, uint256 signerCount);
    event MigrationValidationFailed(string reason);

    /**
     * @notice Run the migration
     * @param oldVerifierAddress Address of the existing InputVerifier
     * @param newThreshold New threshold (must be >= 51% of signers)
     */
    function run(address oldVerifierAddress, uint256 newThreshold) external {
        // Load environment variables
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        verifyingContractSource = vm.envAddress("VERIFYING_CONTRACT_SOURCE");
        chainIDSource = vm.envUint("CHAIN_ID_SOURCE");

        vm.startBroadcast(deployerPrivateKey);

        // Step 1: Validate old verifier
        _validateOldVerifier(oldVerifierAddress);

        // Step 2: Extract current state
        (address[] memory currentSigners, uint256 currentThreshold) = _extractState(oldVerifierAddress);

        // Step 3: Validate new threshold
        _validateNewThreshold(currentSigners.length, newThreshold);

        // Step 4: Deploy new verifier
        EnhancedInputVerifier newVerifier = _deployNewVerifier(currentSigners, newThreshold);

        // Step 5: Verify deployment
        _verifyDeployment(newVerifier, currentSigners, newThreshold);

        // Step 6: Log results
        _logMigration(oldVerifierAddress, address(newVerifier), newThreshold, currentSigners.length);

        vm.stopBroadcast();
    }

    /**
     * @notice Validate the old verifier contract
     */
    function _validateOldVerifier(address oldVerifierAddress) internal view {
        require(oldVerifierAddress != address(0), "Invalid old verifier address");
        require(oldVerifierAddress.code.length > 0, "Old verifier has no code");

        // Check if it's a valid InputVerifier
        try InputVerifier(oldVerifierAddress).getThreshold() returns (uint256 threshold) {
            require(threshold > 0, "Old verifier has zero threshold");
        } catch {
            revert("Old verifier is not a valid InputVerifier");
        }
    }

    /**
     * @notice Extract current state from old verifier
     */
    function _extractState(address oldVerifierAddress)
        internal
        view
        returns (address[] memory signers, uint256 threshold)
    {
        InputVerifier oldVerifier = InputVerifier(oldVerifierAddress);

        // Get current threshold
        threshold = oldVerifier.getThreshold();

        // Get signers (this is a simplified version - actual implementation
        // would need to iterate through signers if there's a getter)
        // For now, we'll need to provide signers manually or through another mechanism

        // Note: The actual implementation depends on InputVerifier's interface
        // This is a placeholder that should be adapted to the actual contract

        return (signers, threshold);
    }

    /**
     * @notice Validate the new threshold
     */
    function _validateNewThreshold(uint256 signerCount, uint256 newThreshold) internal pure {
        require(signerCount >= 3, "Insufficient signers for migration");

        uint256 minThreshold = (signerCount * 51 + 99) / 100;
        require(newThreshold >= minThreshold, "New threshold below minimum");
        require(newThreshold <= signerCount, "New threshold exceeds signer count");

        console.log("Validation passed:");
        console.log("  Signer count:", signerCount);
        console.log("  New threshold:", newThreshold);
        console.log("  Minimum required:", minThreshold);
    }

    /**
     * @notice Deploy the new EnhancedInputVerifier
     */
    function _deployNewVerifier(address[] memory signers, uint256 threshold) internal returns (EnhancedInputVerifier) {
        console.log("Deploying EnhancedInputVerifier...");

        EnhancedInputVerifier newVerifier = new EnhancedInputVerifier();

        newVerifier.initializeFromEmptyProxy(verifyingContractSource, chainIDSource, signers, threshold);

        console.log("Deployed at:", address(newVerifier));

        return newVerifier;
    }

    /**
     * @notice Verify the deployment
     */
    function _verifyDeployment(
        EnhancedInputVerifier newVerifier,
        address[] memory expectedSigners,
        uint256 expectedThreshold
    ) internal view {
        // Verify threshold
        require(newVerifier.getThreshold() == expectedThreshold, "Threshold mismatch");

        // Verify signer count
        require(newVerifier.getCoprocessorSigners().length == expectedSigners.length, "Signer count mismatch");

        // Verify minimum threshold calculation
        uint256 expectedMinThreshold = (expectedSigners.length * 51 + 99) / 100;
        require(newVerifier.getMinimumThreshold() == expectedMinThreshold, "Minimum threshold calculation mismatch");

        console.log("Deployment verified successfully");
    }

    /**
     * @notice Log migration results
     */
    function _logMigration(address oldVerifier, address newVerifier, uint256 threshold, uint256 signerCount)
        internal
        pure
    {
        console.log("\n=== Migration Summary ===");
        console.log("Old Verifier:", oldVerifier);
        console.log("New Verifier:", newVerifier);
        console.log("Threshold:", threshold);
        console.log("Signer Count:", signerCount);
        console.log("Security Level: ENHANCED (51% minimum threshold)");
        console.log("========================\n");
    }
}

/**
 * @title EnhancedInputVerifierUpgrade
 * @notice UUPS upgrade script for existing proxy deployments
 */
contract EnhancedInputVerifierUpgrade is Script {
    /**
     * @notice Upgrade an existing proxy to EnhancedInputVerifier
     * @param proxyAddress Address of the UUPS proxy
     */
    function run(address proxyAddress) external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy new implementation
        EnhancedInputVerifier newImplementation = new EnhancedInputVerifier();

        console.log("New implementation deployed at:", address(newImplementation));

        // Note: Actual upgrade would require calling upgradeTo on the proxy
        // This requires the proxy to be upgradeable and the caller to have permissions

        vm.stopBroadcast();
    }
}

/**
 * @title EnhancedInputVerifierValidation
 * @notice Post-deployment validation script
 */
contract EnhancedInputVerifierValidation is Script {
    /**
     * @notice Validate a deployed EnhancedInputVerifier
     * @param verifierAddress Address of the deployed verifier
     */
    function run(address verifierAddress) external view {
        require(verifierAddress != address(0), "Invalid address");
        require(verifierAddress.code.length > 0, "No code at address");

        EnhancedInputVerifier verifier = EnhancedInputVerifier(verifierAddress);

        console.log("\n=== Validation Report ===");

        // Check threshold
        uint256 threshold = verifier.getThreshold();
        console.log("Current Threshold:", threshold);

        // Check signers
        address[] memory signers = verifier.getCoprocessorSigners();
        console.log("Signer Count:", signers.length);

        // Check minimum threshold
        uint256 minThreshold = verifier.getMinimumThreshold();
        console.log("Minimum Threshold:", minThreshold);

        // Validate security
        require(threshold >= minThreshold, "SECURITY: Threshold below minimum!");
        require(signers.length >= 3, "SECURITY: Insufficient signers!");

        console.log("\nSecurity Status: PASSED");
        console.log("========================\n");
    }
}
