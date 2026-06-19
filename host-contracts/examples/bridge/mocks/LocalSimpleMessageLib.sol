// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {IMessageLib, MessageLibType} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLib.sol";
import {SetConfigParam} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import {Packet} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import {
    ILayerZeroEndpointV2,
    MessagingFee,
    Origin
} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import {PacketV1Codec} from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";

/// @dev Test-only forge-std-free LayerZero V2 send/receive lib for local e2e bridging
///      (LayerZero's SimpleMessageLibMock pulls forge-std and won't compile here). No
///      DVN/Executor: `send` encodes the packet at zero fee for the endpoint to emit as
///      PacketSent; the relayer then calls `validatePacket` on the destination so
///      `endpoint.lzReceive` can deliver it. Registered as the default lib per chain.
contract LocalSimpleMessageLib is ERC165 {
    using PacketV1Codec for bytes;

    address public immutable endpoint;

    error OnlyEndpoint();

    constructor(address _endpoint) {
        endpoint = _endpoint;
    }

    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return interfaceId == type(IMessageLib).interfaceId || super.supportsInterface(interfaceId);
    }

    /// @dev Destination relayer entrypoint: commit a source packet so lzReceive can deliver it.
    function validatePacket(bytes calldata packetBytes) external {
        Origin memory origin = Origin(packetBytes.srcEid(), packetBytes.sender(), packetBytes.nonce());
        ILayerZeroEndpointV2(endpoint).verify(origin, packetBytes.receiverB20(), keccak256(packetBytes.payload()));
    }

    // ---- send library surface called by EndpointV2Mock ----
    function send(
        Packet calldata _packet,
        bytes calldata /*_options*/,
        bool /*_payInLzToken*/
    ) external view returns (MessagingFee memory fee, bytes memory encodedPacket) {
        if (msg.sender != endpoint) revert OnlyEndpoint();
        encodedPacket = PacketV1Codec.encode(_packet);
        fee = MessagingFee(0, 0);
    }

    function quote(
        Packet calldata /*_packet*/,
        bytes calldata /*_options*/,
        bool /*_payInLzToken*/
    ) external pure returns (MessagingFee memory) {
        return MessagingFee(0, 0);
    }

    // ---- IMessageLib views used by the endpoint's library manager ----
    function isSupportedEid(uint32) external pure returns (bool) {
        return true;
    }

    function version() external pure returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (0, 0, 2);
    }

    function messageLibType() external pure returns (MessageLibType) {
        return MessageLibType.SendAndReceive;
    }

    function setConfig(address, SetConfigParam[] calldata) external {}

    function getConfig(uint32, address, uint32) external pure returns (bytes memory) {
        return "";
    }
}
