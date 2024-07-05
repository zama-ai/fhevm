// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../lib/KMSVerifier.sol";

contract GatewayContract is Ownable2Step {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "GatewayContract";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    KMSVerifier immutable kmsVerifier;

    uint256 public constant MAX_DELAY = 1 days;

    struct DecryptionRequest {
        uint256[] cts;
        address contractCaller;
        bytes4 callbackSelector;
        uint256 msgValue;
        uint256 maxTimestamp;
        bool passSignaturesToCaller;
    }

    uint256 public counter; // tracks the number of decryption requests

    mapping(address => bool) public isRelayer;
    mapping(uint256 => DecryptionRequest) internal decryptionRequests;
    mapping(uint256 => bool) internal isFulfilled;

    event EventDecryption(
        uint256 indexed requestID,
        uint256[] cts,
        address contractCaller,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp,
        bool passSignaturesToCaller
    );

    event AddedRelayer(address indexed realyer);

    event RemovedRelayer(address indexed realyer);

    event ResultCallback(uint256 indexed requestID, bool success, bytes result);

    constructor(address _gatewayOwner, address _kmsVerifier) Ownable(_gatewayOwner) {
        kmsVerifier = KMSVerifier(_kmsVerifier);
    }

    modifier onlyRelayer() {
        require(isRelayer[msg.sender], "Not relayer");
        _;
    }

    function addRelayer(address relayerAddress) external onlyOwner {
        require(!isRelayer[relayerAddress], "Address is already relayer");
        isRelayer[relayerAddress] = true;
        emit AddedRelayer(relayerAddress);
    }

    function removeRelayer(address relayerAddress) external onlyOwner {
        require(isRelayer[relayerAddress], "Address is not a relayer");
        isRelayer[relayerAddress] = false;
        emit RemovedRelayer(relayerAddress);
    }

    function isExpiredOrFulfilled(uint256 requestID) external view returns (bool) {
        uint256 timeNow = block.timestamp;
        return
            isFulfilled[requestID] ||
            (timeNow > decryptionRequests[requestID].maxTimestamp && decryptionRequests[requestID].maxTimestamp != 0);
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
    ) external returns (uint256 initialCounter) {
        require(maxTimestamp > block.timestamp, "maxTimestamp must be a future date");
        require(maxTimestamp <= block.timestamp + MAX_DELAY, "maxTimestamp exceeded MAX_DELAY");
        initialCounter = counter;
        DecryptionRequest storage decryptionReq = decryptionRequests[initialCounter];

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
        counter++;
    }

    // Called by the relayer to pass the decryption results from the KMS to the callback function
    // `decryptedCts` is a bytes array containing the abi-encoded concatenation of decryption results.
    function fulfillRequest(
        uint256 requestID,
        bytes memory decryptedCts,
        bytes[] memory signatures
    ) external payable onlyRelayer {
        // TODO: this should be un-commented once KMS will have the signatures implemented
        //require(
        //    kmsVerifier.verifySignatures(decryptionRequests[requestID].cts, decryptedCts, signatures),
        //    "KMS signature verification failed"
        //);
        require(!isFulfilled[requestID], "Request is already fulfilled");
        DecryptionRequest memory decryptionReq = decryptionRequests[requestID];
        require(block.timestamp <= decryptionReq.maxTimestamp, "Too late");
        bytes memory callbackCalldata = abi.encodeWithSelector(decryptionReq.callbackSelector, requestID);
        bool passSignatures = decryptionReq.passSignaturesToCaller;
        callbackCalldata = abi.encodePacked(callbackCalldata, decryptedCts); // decryptedCts MUST be correctly abi-encoded by the relayer, according to the requested types of `ctsHandles`
        if (passSignatures) {
            callbackCalldata = abi.encodePacked(callbackCalldata, abi.encode(signatures));
        }
        (bool success, bytes memory result) = (decryptionReq.contractCaller).call{value: decryptionReq.msgValue}(
            callbackCalldata
        );
        emit ResultCallback(requestID, success, result);
        isFulfilled[requestID] = true;
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure returns (string memory) {
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
