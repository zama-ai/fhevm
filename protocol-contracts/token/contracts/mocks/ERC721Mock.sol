// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract ERC721Mock is ERC721 {
    constructor(string memory _name, string memory _symbol) ERC721(_name, _symbol) {}

    function mint(address _to, uint256 _tokenId) public {
        _mint(_to, _tokenId);
    }
}
