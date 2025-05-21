// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EIP712UpgradeableCrossChain} from "../../contracts/EIP712UpgradeableCrossChain.sol";

contract MockEIP712UpgradeableCrossChain is UUPSUpgradeable, EIP712UpgradeableCrossChain {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @dev This function returns the domain separator.
    function viewDomainSeparator() public view returns (bytes32) {
        return _domainSeparatorV4();
    }

    /// @dev This function returns the EIP712 name hash.
    function viewEIP712NameHash() public view returns (bytes32) {
        return _EIP712NameHash();
    }

    /// @dev This function returns the EIP712 version hash.
    function viewEIP712VersionHash() public view returns (bytes32) {
        return _EIP712VersionHash();
    }

    /// @dev  Re-initializes the contract.
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitialize(address verifyingContractSource, uint64 chainIDSource) public virtual reinitializer(2) {
        __EIP712_init("MockEIP712UpgradeableCrossChain", "1", verifyingContractSource, chainIDSource);
    }

    /// @dev This function overrides the storage slots of the contract for testing purposes.
    function overrideStorageSlots(
        bytes32 hashedName,
        bytes32 hashedVersion,
        address verifyingContractSource,
        uint64 chainIDsource,
        string memory name,
        string memory version
    ) public {
        EIP712Storage storage $;

        bytes32 EIP712UpgradeableCrossChainLocation = keccak256(
            abi.encode(uint256(keccak256("fhevm.storage.EIP712UpgradeableCrossChain")) - 1)
        ) & ~bytes32(uint256(0xff));

        assembly {
            $.slot := EIP712UpgradeableCrossChainLocation
        }

        $._hashedName = hashedName;
        $._hashedVersion = hashedVersion;
        $._verifyingContractSource = verifyingContractSource;
        $._chainIDSource = chainIDsource;
        $._name = name;
        $._version = version;
    }
    /// @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
    function _authorizeUpgrade(address _newImplementation) internal virtual override {}
}

contract EIP712UpgradeableCrossChainTest is Test {
    /// @dev Proxy and implementation variables
    address internal proxy;
    address internal owner = address(456);

    MockEIP712UpgradeableCrossChain internal mockEIP712UpgradeableCrossChain;

    /**
     * @dev Internal function to deploy a UUPS proxy contract.
     * The proxy is deployed using the UnsafeUpgrades library and initialized with the owner address.
     */
    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );
    }

    /**
     * @dev Internal function to update the proxy contract.
     * The proxy is upgraded to a new implementation of MockEIP712UpgradeableCrossChain and reinitialized with the given parameters.
     * @param verificationContractSource The address of the verification contract source.
     * @param chainIdSource The chain ID source.
     */
    function _updateProxy(address verificationContractSource, uint64 chainIdSource) internal {
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new MockEIP712UpgradeableCrossChain()),
            abi.encodeCall(MockEIP712UpgradeableCrossChain.reinitialize, (verificationContractSource, chainIdSource)),
            owner
        );

        mockEIP712UpgradeableCrossChain = MockEIP712UpgradeableCrossChain(proxy);
    }

    function setUp() public {
        _deployProxy();
    }

    /**
     * @dev Test function to check the initialization of the EIP712 domain.
     * It verifies that the domain separator is correctly set and that the fields are as expected.
     */
    function test_PostInitialization() public {
        address verificationContractSource = address(123);
        uint64 chainIdSource = uint64(block.chainid);
        _updateProxy(verificationContractSource, chainIdSource);
        (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        ) = mockEIP712UpgradeableCrossChain.eip712Domain();
        assertEq(fields, hex"0f");
        assertEq(name, "MockEIP712UpgradeableCrossChain");
        assertEq(version, "1");
        assertEq(chainId, chainIdSource);
        assertEq(verifyingContract, verificationContractSource);
        assertEq(salt, bytes32(0));
        assertEq(extensions.length, 0);

        bytes32 domainSeparator = mockEIP712UpgradeableCrossChain.viewDomainSeparator();

        assertEq(
            domainSeparator,
            keccak256(
                abi.encode(
                    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                    keccak256(bytes("MockEIP712UpgradeableCrossChain")),
                    keccak256(bytes("1")),
                    chainIdSource,
                    verificationContractSource
                )
            )
        );
    }

    /**
     * @dev Test function to check the initialization of the EIP712 domain with empty arguments.
     * It verifies that the domain separator is correctly set and that the fields are as expected.
     */
    function test_ReturnsWhatIsExpectedIfInitializedWithoutArguments() public {
        UnsafeUpgrades.upgradeProxy(proxy, address(new MockEIP712UpgradeableCrossChain()), abi.encode(), owner);

        mockEIP712UpgradeableCrossChain = MockEIP712UpgradeableCrossChain(proxy);
        mockEIP712UpgradeableCrossChain.eip712Domain();

        (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        ) = mockEIP712UpgradeableCrossChain.eip712Domain();
        assertEq(fields, hex"0f");
        assertEq(name, "");
        assertEq(version, "");
        assertEq(chainId, 0);
        assertEq(verifyingContract, address(0));
        assertEq(salt, bytes32(0));
        assertEq(extensions.length, 0);
    }

    /**
     * @dev Tests that it reverts if EIP712 domain is not initialized properly
     */
    function test_CannotCallEIP712DomainIfNotInitialized() public {
        address verificationContractSource = address(123);
        uint64 chainIdSource = uint64(block.chainid);

        _updateProxy(verificationContractSource, chainIdSource);

        bytes32 hashedVersion;
        bytes32 hashedName;
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(hashedName, hashedVersion, address(0), 0, "", "");
        mockEIP712UpgradeableCrossChain.eip712Domain();

        /// @dev It reverts if hashedName is set.
        hashedName = keccak256(abi.encodePacked("1"));
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(hashedName, hashedVersion, address(0), 0, "", "");
        vm.expectRevert("EIP712: Uninitialized");
        mockEIP712UpgradeableCrossChain.eip712Domain();

        /// @dev It reverts if hashedVersion is set.
        hashedName = 0;
        hashedVersion = keccak256(abi.encodePacked("1"));
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(hashedName, hashedVersion, address(0), 0, "", "");
        vm.expectRevert("EIP712: Uninitialized");
        mockEIP712UpgradeableCrossChain.eip712Domain();
    }

    /**
     * @dev Tests that it returns what is expected if hashedName is set but not the name.
     */
    function test_EIP712NameHashIsCustomIfNameIsEmptyButHashedNameInStorageIfNonZero() public {
        address verificationContractSource = address(123);
        uint64 chainIdSource = uint64(block.chainid);
        _updateProxy(verificationContractSource, chainIdSource);

        bytes32 hashedVersion;
        bytes32 hashedName;
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(
            hashedName,
            hashedVersion,
            verificationContractSource,
            chainIdSource,
            "",
            ""
        );

        bytes32 eip712DomainHash = mockEIP712UpgradeableCrossChain.viewEIP712NameHash();
        assertEq(eip712DomainHash, keccak256(""));

        hashedName = keccak256(abi.encodePacked("1"));
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(
            hashedName,
            hashedVersion,
            verificationContractSource,
            chainIdSource,
            "",
            ""
        );

        eip712DomainHash = mockEIP712UpgradeableCrossChain.viewEIP712NameHash();
        assertEq(eip712DomainHash, hashedName);
    }

    /**
     * @dev Tests that it returns what is expected if hashedVersion is set but not the version.
     */
    function test_EIP712VersionHashIsCustomIfVersionIsEmptyButHashedVersionInStorageIfNonZero() public {
        address verificationContractSource = address(123);
        uint64 chainIdSource = uint64(block.chainid);
        _updateProxy(verificationContractSource, chainIdSource);

        bytes32 hashedVersion;
        bytes32 hashedName;
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(
            hashedName,
            hashedVersion,
            verificationContractSource,
            chainIdSource,
            "",
            ""
        );

        bytes32 eip712DomainHash = mockEIP712UpgradeableCrossChain.viewEIP712VersionHash();
        assertEq(eip712DomainHash, keccak256(""));

        hashedVersion = keccak256(abi.encodePacked("1"));
        mockEIP712UpgradeableCrossChain.overrideStorageSlots(
            hashedName,
            hashedVersion,
            verificationContractSource,
            chainIdSource,
            "",
            ""
        );

        eip712DomainHash = mockEIP712UpgradeableCrossChain.viewEIP712VersionHash();
        assertEq(eip712DomainHash, hashedVersion);
    }
}
