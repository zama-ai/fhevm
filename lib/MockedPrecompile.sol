// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

contract MockedPrecompile {
    uint8 public constant HANDLE_VERSION = 0;

    /// @dev handle format for user inputs is: keccak256(keccak256(CiphertextFHEList)||index_handle)[0:29] || index_handle || handle_type || handle_version
    /// @dev other handles format (fhe ops results) is: keccak256(keccak256(rawCiphertextFHEList)||index_handle)[0:30] || handle_type || handle_version
    /// @dev the CiphertextFHEList actually contains: 1 byte (= N) for size of handles_list, N bytes for the handles_types : 1 per handle, then the original fhe160list raw ciphertext
    function typeOf(uint256 handle) internal pure returns (uint8) {
        uint8 typeCt = uint8(handle >> 8);
        return typeCt;
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address /*callerAddress*/,
        address /*contractAddress*/,
        bytes memory inputProof,
        bytes1 handleType
    ) external pure returns (uint256 result) {
        result = uint256(inputHandle);
        uint256 indexHandle = (result & 0x0000000000000000000000000000000000000000000000000000000000ff0000) >> 16;
        uint8 typeCt = typeOf(result);

        require(uint8(handleType) == typeCt, "Wrong type");

        uint256 inputProofLen = inputProof.length;
        require(inputProofLen > 0, "Empty inputProof");

        require(uint8(result) == HANDLE_VERSION, "Wrong handle version");

        bytes32 checkHandle = keccak256(abi.encodePacked(keccak256(inputProof), uint8(indexHandle)));
        bytes32 mask = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000;
        // check correct inputHandle was sent, corresponding to the inputProof: the 29 first bytes must be equal
        require((inputHandle & mask) == (checkHandle & mask), "Wrong inputHandle");
    }
}
