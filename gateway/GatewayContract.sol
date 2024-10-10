// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../lib/KMSVerifierAddress.sol";
import "../lib/ACLAddress.sol";
import "./IKMSVerifier.sol";

contract GatewayContract is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "GatewayContract";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    IKMSVerifier private constant kmsVerifier = IKMSVerifier(kmsVerifierAdd);
    address private constant aclAddress = aclAdd;

    uint256 private constant MAX_DELAY = 1 days;

    struct DecryptionRequest {
        uint256[] cts;
        address contractCaller;
        bytes4 callbackSelector;
        uint256 msgValue;
        uint256 maxTimestamp;
        bool passSignaturesToCaller;
    }

    event EventDecryption(
        uint256 indexed requestID,
        uint256[] cts,
        address contractCaller,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp,
        bool passSignaturesToCaller
    );

    event AddedRelayer(address indexed relayer);

    event RemovedRelayer(address indexed relayer);

    event ResultCallback(uint256 indexed requestID, bool success, bytes result);

    function getMAX_DELAY() external virtual returns (uint256) {
        return MAX_DELAY;
    }

    function getKmsVerifierAddress() external virtual returns (address) {
        return kmsVerifierAdd;
    }

    function getCounter() external virtual returns (uint256) {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        return $.counter;
    }

    function isRelayer(address account) external virtual returns (bool) {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        return $.isRelayer[account];
    }

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @custom:storage-location erc7201:fhevm.storage.GatewayContract
    struct GatewayContractStorage {
        uint256 counter; // tracks the number of decryption requests
        mapping(address => bool) isRelayer;
        mapping(uint256 => DecryptionRequest) decryptionRequests;
        mapping(uint256 => bool) isFulfilled;
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.GatewayContract")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant GatewayContractStorageLocation =
        0x2f81b8bba57448689ab73c47570e3de1ee7f779a62f121c9631b35b3eda2aa00;

    function _getGatewayContractStorage() internal pure returns (GatewayContractStorage storage $) {
        assembly {
            $.slot := GatewayContractStorageLocation
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _gatewayOwner) external initializer {
        __Ownable_init(_gatewayOwner);
    }

    modifier onlyRelayer() {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        require($.isRelayer[msg.sender], "Not relayer");
        _;
    }

    function addRelayer(address relayerAddress) external virtual onlyOwner {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        require(!$.isRelayer[relayerAddress], "Address is already relayer");
        $.isRelayer[relayerAddress] = true;
        emit AddedRelayer(relayerAddress);
    }

    function removeRelayer(address relayerAddress) external virtual onlyOwner {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        require($.isRelayer[relayerAddress], "Address is not a relayer");
        $.isRelayer[relayerAddress] = false;
        emit RemovedRelayer(relayerAddress);
    }

    function isExpiredOrFulfilled(uint256 requestID) external view virtual returns (bool) {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        uint256 timeNow = block.timestamp;
        return
            $.isFulfilled[requestID] ||
            (timeNow > $.decryptionRequests[requestID].maxTimestamp &&
                $.decryptionRequests[requestID].maxTimestamp != 0);
    }

    /// @notice Requests the decryption of n ciphertexts `ctsHandles` with the result returned in a callback.
    /// @notice During callback, msg.sender is called with [callbackSelector,requestID,decrypt(ctsHandles[0]),decrypt(ctsHandles[1]),...,decrypt(ctsHandles[n-1]), **signatures**]
    /// @notice as calldata via `fulfillRequest`.
    /// @notice **the last argument `signatures` in calldata is optional, and will only be passed if `passSignaturesToCaller` argument was set to `true`.
    /// @param ctsHandles is an array of uint256s handles.
    /// @param callbackSelector the callback selector to be called on msg.sender later during fulfilment
    /// @param msgValue is msg.value of callback tx, to be used if callback is payable
    /// @param maxTimestamp maximul timestamp to fulfill the request, else callback function will not be called, should not exceed current date + MAX_DELAY
    /// @param passSignaturesToCaller true if you want to additionally pass the signatures at the end of calldata of the callback
    function requestDecryption(
        uint256[] calldata ctsHandles,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp,
        bool passSignaturesToCaller
    ) external virtual returns (uint256 initialCounter) {
        require(maxTimestamp > block.timestamp, "maxTimestamp must be a future date");
        require(maxTimestamp <= block.timestamp + MAX_DELAY, "maxTimestamp exceeded MAX_DELAY");
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        initialCounter = $.counter;
        DecryptionRequest storage decryptionReq = $.decryptionRequests[initialCounter];

        uint256 len = ctsHandles.length;
        for (uint256 i = 0; i < len; i++) {
            decryptionReq.cts.push(ctsHandles[i]);
        }

        decryptionReq.contractCaller = msg.sender;
        decryptionReq.callbackSelector = callbackSelector;
        decryptionReq.msgValue = msgValue;
        decryptionReq.maxTimestamp = maxTimestamp;
        decryptionReq.passSignaturesToCaller = passSignaturesToCaller;
        emit EventDecryption(
            initialCounter,
            ctsHandles,
            msg.sender,
            callbackSelector,
            msgValue,
            maxTimestamp,
            passSignaturesToCaller
        );
        $.counter++;
    }

    // Called by the relayer to pass the decryption results from the KMS to the callback function
    // `decryptedCts` is a bytes array containing the abi-encoded concatenation of decryption results.
    function fulfillRequest(
        uint256 requestID,
        bytes memory decryptedCts,
        bytes[] memory signatures
    ) external payable virtual onlyRelayer {
        GatewayContractStorage storage $ = _getGatewayContractStorage();
        require(
            kmsVerifier.verifyDecryptionEIP712KMSSignatures(
                aclAddress,
                $.decryptionRequests[requestID].cts,
                decryptedCts,
                signatures
            ),
            "KMS signature verification failed"
        );
        require(!$.isFulfilled[requestID], "Request is already fulfilled");
        DecryptionRequest memory decryptionReq = $.decryptionRequests[requestID];
        require(block.timestamp <= decryptionReq.maxTimestamp, "Too late");
        bytes memory callbackCalldata = abi.encodeWithSelector(decryptionReq.callbackSelector, requestID);
        bool passSignatures = decryptionReq.passSignaturesToCaller;
        callbackCalldata = abi.encodePacked(callbackCalldata, decryptedCts); // decryptedCts MUST be correctly abi-encoded by the relayer, according to the requested types of `ctsHandles`
        if (passSignatures) {
            bytes memory packedSignatures = abi.encode(signatures);
            bytes memory packedSignaturesNoOffset = removeOffset(packedSignatures); // remove the offset (the first 32 bytes) before concatenating with the first part of calldata
            callbackCalldata = abi.encodePacked(callbackCalldata, packedSignaturesNoOffset);
        }

        (bool success, bytes memory result) = (decryptionReq.contractCaller).call{value: decryptionReq.msgValue}(
            callbackCalldata
        );
        emit ResultCallback(requestID, success, result);
        $.isFulfilled[requestID] = true;
    }

    function removeOffset(bytes memory input) public pure virtual returns (bytes memory) {
        uint256 newLength = input.length - 32;
        bytes memory result = new bytes(newLength);
        for (uint256 i = 0; i < newLength; i++) {
            result[i] = input[i + 32];
        }
        return result;
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
