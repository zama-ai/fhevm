// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Script, console} from "forge-std-1.11.0/Script.sol";
import {FHETest} from "../src/FHETest.sol";
import {LocalConfig} from "../src/LocalConfig.sol";
import {SepoliaConfig} from "../src/SepoliaConfig.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

string constant DEFAULT_CHAIN = "localhostFhevm";
string constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";
address constant FHE_TEST_DEVNET = 0xD26bB032e2F06A5382902559c4EbBB82C35C6dDF;

// EIP-1967 implementation storage slot
bytes32 constant IMPL_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

interface IVersioned {
    function getVersion() external view returns (string memory);
    function CONTRACT_NAME() external view returns (string memory);
    function eip712Domain()
        external
        view
        returns (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        );
}

abstract contract FHETestScript is Script {
    uint256 internal deployerPrivateKey;
    address internal deployer;
    address internal expectedFheTestAddr;
    CoprocessorConfig internal coprocessorConfig;

    function setUp() public virtual {
        // Load chain-specific .env file based on FHEVM env var
        uint32 index = uint32(vm.envOr("MNEMONIC_INDEX", uint256(0)));
        string memory mnemonic = vm.envOr("MNEMONIC", FHEVM_MNEMONIC);
        string memory fhevmChain = vm.envOr("CHAIN", DEFAULT_CHAIN);
        expectedFheTestAddr = address(vm.envOr("FHE_TEST", address(0)));

        // forge-lint: disable-next-line(unsafe-cheatcode)
        deployerPrivateKey = vm.deriveKey(mnemonic, index);
        deployer = vm.addr(deployerPrivateKey);

        uint64 nonce = 0;

        console.log("Fhevm Chain:        ", fhevmChain);
        console.log("Deployer:           ", deployer);

        if (block.chainid == 11155111) {
            // deployer must already be funded
            require(deployer.balance >= 0.01 ether, "Deployer has insufficient balance on Sepolia");
            bytes32 fhevmChainHash = keccak256(bytes(fhevmChain));
            if (fhevmChainHash == keccak256("devnet")) {
                // Sepolia (Devnet)
                coprocessorConfig = SepoliaConfig.getDevnetConfig();
                expectedFheTestAddr = FHE_TEST_DEVNET;
            } else if (fhevmChainHash == keccak256("testnet")) {
                // Sepolia (Testnet)
                coprocessorConfig = SepoliaConfig.getTestnetConfig();
            } else {
                revert(string.concat("Unsupported CHAIN value for Sepolia: \"", fhevmChain, "\". Expected \"devnet\" or \"testnet\"."));
            }

            if (expectedFheTestAddr == address(0)) {
                nonce = vm.getNonce(deployer);
                console.log("Deployer nonce:     ", nonce);
                // Compute deterministic deploy address using the current nonce
                expectedFheTestAddr = vm.computeCreateAddress(deployer, nonce);
            }
        } else {
            require(nonce == 0, "Local anvil deployer must have nonce 0 for deterministic deploy");

            // Local anvil — fund deployer via cheat code
            if (deployer.balance < 1 ether) {
                string memory balance = vm.toString(uint256(100 ether));
                vm.rpc("anvil_setBalance", string.concat('["', vm.toString(deployer), '", "', balance, '"]'));
            }
            coprocessorConfig = LocalConfig.getLocalConfig();
        }

        console.log("Expected FHETest at:", expectedFheTestAddr);

        console.log("Verifying coprocessor contracts...");
        
        verifyProxy("ACL", coprocessorConfig.ACLAddress, false);
        verifyProxy("Coprocessor", coprocessorConfig.CoprocessorAddress, false);
        verifyProxy("KMSVerifier", coprocessorConfig.KMSVerifierAddress, true);
    }

    function verifyProxy(string memory label, address proxy, bool eip712) internal view {
        // Verify proxy has code
        require(proxy.code.length > 0, string.concat(label, ": no code at proxy address"));

        // Read EIP-1967 implementation slot
        bytes32 implSlotValue = vm.load(proxy, IMPL_SLOT);
        address impl = address(uint160(uint256(implSlotValue)));
        require(impl != address(0), string.concat(label, ": not an EIP-1967 proxy (impl is zero)"));
        require(impl.code.length > 0, string.concat(label, ": no code at implementation address"));

        console.log(string.concat("  ", label, ":"));
        console.log("    proxy:", proxy);
        console.log("    impl: ", impl);

        // Call getVersion()
        IVersioned v = IVersioned(proxy);
        console.log("    getVersion():    ", v.getVersion());

        if (eip712) {
            // Call eip712Domain()
            (, string memory name, string memory version, uint256 chainId, address verifyingContract,,) = v.eip712Domain();
            console.log("    eip712Domain():");
            console.log("      name:             ", name);
            console.log("      version:          ", version);
            console.log("      chainId:          ", chainId);
            console.log("      verifyingContract:", verifyingContract);
        }
    }

    function verifyCoprocessorConfig(FHETest fheTest) internal view {
        CoprocessorConfig memory actual = fheTest.getCoprocessorConfig();

        require(actual.ACLAddress == coprocessorConfig.ACLAddress, "ACL address mismatch");
        require(actual.CoprocessorAddress == coprocessorConfig.CoprocessorAddress, "Coprocessor address mismatch");
        require(actual.KMSVerifierAddress == coprocessorConfig.KMSVerifierAddress, "KMSVerifier address mismatch");

        console.log("Verifying coprocessor contracts...");
        verifyProxy("ACL", actual.ACLAddress, false);
        verifyProxy("Coprocessor", actual.CoprocessorAddress, false);
        verifyProxy("KMSVerifier", actual.KMSVerifierAddress, true);
    }
}
